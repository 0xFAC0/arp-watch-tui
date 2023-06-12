use pnet::{
    datalink::{self, Channel, DataLinkReceiver, DataLinkSender, NetworkInterface},
    packet::{
        arp::{ArpHardwareTypes, ArpOperations, ArpPacket, MutableArpPacket},
        ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket},
        MutablePacket, Packet,
    },
    util::MacAddr,
};
use std::{
    net::{IpAddr, Ipv4Addr},
    thread::{self, JoinHandle},
};

use crate::arp_cache::*;

pub struct Sniffer {
    tx: Box<dyn DataLinkSender>,
    pub rx_th: JoinHandle<()>,
    iface: NetworkInterface,
    source_mac: MacAddr,
    source_ip: Ipv4Addr,
}

fn get_interface(iface_name: String) -> NetworkInterface {
    let iface_name_filter = |iface: &NetworkInterface| iface.name == iface_name;
    match datalink::interfaces().into_iter().find(iface_name_filter) {
        Some(iface) => iface,
        None => panic!("Interface not found: {}", iface_name),
    }
}

fn get_default_interface() -> NetworkInterface {
    for interface in datalink::interfaces().into_iter() {
        if !interface.is_loopback() {
            return interface;
        }
    }
    panic!("No avaible network interface");
}

impl Sniffer {
    pub fn new(
        iface_name: Option<String>,
        arp_cache: ArpCacheMutex,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let iface = match iface_name {
            Some(iface_name) => get_interface(iface_name),
            None => get_default_interface(),
        };

        let _source_mac = iface.mac.unwrap();
        let source_mac = iface.mac.unwrap();

        let (tx, rx) = Sniffer::get_iface_channel(&iface);
        let source_ip = match iface.ips.first().unwrap().ip() {
            IpAddr::V4(addr) => addr,
            IpAddr::V6(_) => panic!("Ipv6 unsupported"),
        };

        let rx_th = thread::spawn(move || Sniffer::recv_thread(rx, arp_cache));

        Ok(Sniffer {
            tx,
            rx_th,
            iface,
            source_mac,
            source_ip,
        })
    }

    fn get_iface_channel(
        iface: &NetworkInterface,
    ) -> (Box<dyn DataLinkSender>, Box<dyn DataLinkReceiver>) {
        match pnet::datalink::channel(iface, Default::default()) {
            Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unknown channel type"),
            Err(e) => panic!("Pnet channel: {}", e),
        }
    }

    fn recv_thread(mut rx: Box<dyn DataLinkReceiver>, arp_cache: ArpCacheMutex) {
        let update_cache = |sender_ip: Ipv4Addr, sender_mac: MacAddr| {
            let new_entry = ArpEntry::new(sender_ip, sender_mac);

            let mut arp_cache_lock = arp_cache.lock().unwrap();
            arp_cache_lock.update(new_entry);
            drop(arp_cache_lock);
        };

        loop {
            if let Ok(buf) = rx.next() {
                let ethernet_packet = match EthernetPacket::new(buf) {
                    Some(ethernet_packet) => ethernet_packet,
                    None => {
                        println!("Invalid ethernet packet ?!\n{:#?}", buf);
                        continue;
                    }
                };
                if ethernet_packet.get_ethertype() != EtherTypes::Arp {
                    continue;
                }

                let arp_packet = match ArpPacket::new(ethernet_packet.payload()) {
                    Some(arp_packet) => arp_packet,
                    None => {
                        println!("Invalid Ethernet ARP Payload");
                        continue;
                    }
                };
                let operation = arp_packet.get_operation();
                let _target_mac = arp_packet.get_target_hw_addr();
                let target_ip = arp_packet.get_target_proto_addr();
                let sender_mac = arp_packet.get_sender_hw_addr();
                let sender_ip = arp_packet.get_sender_proto_addr();

                if operation == ArpOperations::Reply {
                    println!("Got ARP Reply");
                    println!("[{}]: {} is at {}", sender_mac, sender_ip, sender_mac);
                    update_cache(sender_ip, sender_mac);
                }
                if operation == ArpOperations::Request && sender_ip == target_ip {
                    println!("Got ARP Annoncement");
                    println!("[{}] {} at {}", sender_mac, sender_ip, sender_mac,);
                    update_cache(sender_ip, sender_mac);
                }
            }
        }
    }

    pub fn scan_network(&mut self) {
        let network_addr = self.iface.ips.first().unwrap();
        println!("Starting host scan on {}", network_addr);

        // Very nice network address range traversal from ipnetwork crate
        for target_ip in network_addr.iter() {
            // Unwrapp IpAddr to Ipv4Addr
            let target_ip = match target_ip {
                IpAddr::V4(addr) => addr,
                IpAddr::V6(_) => panic!("Ipv6 unsupported yet"),
            };

            let mut ethernet_buffer = [0u8; 42];
            let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

            ethernet_packet.set_destination(MacAddr::broadcast());
            ethernet_packet.set_source(self.source_mac);
            ethernet_packet.set_ethertype(EtherTypes::Arp);

            let mut arp_buffer = [0u8; 28];
            let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

            arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
            arp_packet.set_protocol_type(EtherTypes::Ipv4);
            arp_packet.set_hw_addr_len(6);
            arp_packet.set_proto_addr_len(4);
            arp_packet.set_operation(ArpOperations::Request);
            arp_packet.set_sender_hw_addr(self.source_mac);
            arp_packet.set_sender_proto_addr(self.source_ip);
            arp_packet.set_target_hw_addr(MacAddr::broadcast());
            arp_packet.set_target_proto_addr(target_ip);

            // Smooth
            ethernet_packet.set_payload(arp_packet.packet_mut());

            // Comment form pnet: The second parameter is ignored but None must still be passed
            // Send to wrap 2 results, pnet scope error and std io's result
            self.tx
                .send_to(ethernet_packet.packet(), None)
                .unwrap()
                .unwrap();
        }
        println!("Done sending arp request");
    }
}
