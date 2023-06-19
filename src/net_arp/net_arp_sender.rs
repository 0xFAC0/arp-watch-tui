use std::net::IpAddr;

use log::info;
use pnet::{
    datalink::{self, Channel::Ethernet, NetworkInterface},
    packet::{
        arp::{ArpHardwareTypes, ArpOperation, ArpOperations, MutableArpPacket},
        ethernet::{EtherTypes, MutableEthernetPacket},
        Packet,
    },
    util::MacAddr,
};

use crate::arp_cache::ArpEntry;

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

    pub fn build_packet<'a>(
        operation: ArpOperation,
        ip_source: Ipv4Addr,
        hw_source: MacAddr,
        ip_target: Ipv4Addr,
        hw_target: MacAddr,
    ) -> MutableEthernetPacket<'a> {
        let ethernet_buffer = Vec::from([0u8; 42]);
        let mut ethernet_packet = MutableEthernetPacket::owned(ethernet_buffer).unwrap();

        ethernet_packet.set_destination(MacAddr::broadcast());
        ethernet_packet.set_source(hw_source);
        ethernet_packet.set_ethertype(EtherTypes::Arp);

        let arp_buffer = Vec::from([0u8; 28]);

        // TODO Error invalid ARP packet when Option is None
        let mut arp_packet = MutableArpPacket::owned(arp_buffer).unwrap();

        arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
        arp_packet.set_protocol_type(EtherTypes::Ipv4);
        arp_packet.set_hw_addr_len(6);
        arp_packet.set_proto_addr_len(4);
        arp_packet.set_operation(operation);
        arp_packet.set_sender_hw_addr(hw_source);
        arp_packet.set_sender_proto_addr(ip_source);
        arp_packet.set_target_hw_addr(hw_target);
        arp_packet.set_target_proto_addr(ip_target);

        ethernet_packet.set_payload(arp_packet.packet());
        ethernet_packet
    }

    pub fn rearp(&mut self, entry: ArpEntry) {
        info!("ReARPing");
        let packet = Self::build_packet(
            ArpOperations::Request,
            *entry.ip(),
            *entry.mac(),
            self.source_ip,
            self.source_mac,
        );
        self.tx.send_to(packet.packet(), None).unwrap().unwrap();
    }

    pub fn scan_network(&mut self) -> std::io::Result<()> {
        info!("Starting host scan on {}", self.network_addr);

        // Very nice network address range traversal from ipnetwork
        for target_ip in self.network_addr.iter() {
            // Unwrapp IpAddr to Ipv4Addr
            let target_ip = match target_ip {
                IpAddr::V4(addr) => addr,
                // TODO Implement Ipv6
                IpAddr::V6(_) => panic!("Ipv6 unsupported yet"),
            };

            let packet = Self::build_packet(
                ArpOperations::Request,
                self.source_ip,
                self.source_mac,
                target_ip,
                MacAddr::broadcast(),
            );

            self.tx.send_to(packet.packet(), None).unwrap()?;
        }
        info!("Done sending arp request");
        Ok(())
    }
}
