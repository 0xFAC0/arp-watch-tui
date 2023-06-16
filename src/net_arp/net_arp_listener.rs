use core::panic;
use std::error::Error;

use log::{warn, info};
use pnet::{
    datalink::{self, Channel::Ethernet, NetworkInterface},
    packet::{
        arp::{ArpOperations, ArpPacket},
        ethernet::{EtherTypes, EthernetPacket},
        Packet,
    },
};

use crate::arp_cache::*;

use super::NetArpListener;

impl NetArpListener {
    pub fn new(interface: &NetworkInterface, arp_cache: ArpCacheMutex) -> Self {
        let (_, rx) = match datalink::channel(interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type"),
            Err(e) => panic!(
                "An error occurred when creating the datalink sender channel: {}",
                e
            ),
        };
        Self { rx, arp_cache }
    }

    pub async fn packet_handler(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            if let Ok(buf) = self.rx.next() {
                let ethernet_packet = match EthernetPacket::new(buf) {
                    Some(ethernet_packet) => ethernet_packet,
                    None => continue,
                };

                if ethernet_packet.get_ethertype() != EtherTypes::Arp {
                    continue;
                }

                let arp_packet = match ArpPacket::new(ethernet_packet.payload()) {
                    Some(arp_packet) => arp_packet,
                    None => continue,
                };
                let operation = arp_packet.get_operation();
                let _target_mac = arp_packet.get_target_hw_addr();
                let target_ip = arp_packet.get_target_proto_addr();
                let sender_mac = arp_packet.get_sender_hw_addr();
                let sender_ip = arp_packet.get_sender_proto_addr();

                if operation == ArpOperations::Reply {
                    info!(
                        "[Listener] ARP Reply\n[Listener] {} is at {}",
                        sender_ip, sender_mac
                    );
                    let mut arp_cache = self.arp_cache.lock().await;
                    arp_cache.update(ArpEntry::new(sender_ip, sender_mac));
                } else if operation == ArpOperations::Request && sender_ip == target_ip {
                    info!(
                        "[Listener] ARP Annoncement\n[Listener] {} is at {}",
                        sender_ip, sender_mac
                    );
                    let mut arp_cache = self.arp_cache.lock().await;
                    arp_cache.update(ArpEntry::new(sender_ip, sender_mac));
                }
            }
        }
    }
}
