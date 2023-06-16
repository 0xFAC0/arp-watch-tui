use std::{error::Error, net::IpAddr};

use log::info;
use pnet::{
    datalink::{self, Channel::Ethernet, NetworkInterface},
    packet::{
        arp::{ArpHardwareTypes, ArpOperations, MutableArpPacket},
        ethernet::{EtherTypes, MutableEthernetPacket},
        MutablePacket, Packet,
    },
    util::MacAddr,
};

use super::*;

impl NetArpSender {
    pub fn new(interface: &NetworkInterface) -> Self {
        let network_addr = interface
            .ips
            .first()
            .expect("No network ip avaible")
            .to_owned();
        let source_ip = match network_addr.ip() {
            IpAddr::V4(ipv4) => ipv4,
            IpAddr::V6(_) => unimplemented!(),
        };
        let source_mac = interface
            .mac
            .unwrap_or_else(|| panic!("No MAC address for {}", interface.name));
        let (tx, _) = match datalink::channel(interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type"),
            Err(e) => panic!(
                "An error occurred when creating the datalink sender channel: {}",
                e
            ),
        };
        Self {
            tx,
            source_mac,
            source_ip,
            network_addr,
        }
    }

    pub async fn scan_network(&mut self) -> std::io::Result<()> {
        info!("Starting host scan on {}", self.network_addr);

        // Very nice network address range traversal from ipnetwork
        for target_ip in self.network_addr.iter() {
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

            self.tx.send_to(ethernet_packet.packet(), None).unwrap()?;
        }
        info!("Done sending arp request");
        Ok(())
    }
}
