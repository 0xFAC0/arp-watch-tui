use std::net::Ipv4Addr;

use pnet::{datalink::{DataLinkSender, DataLinkReceiver, NetworkInterface}, util::MacAddr, ipnetwork::IpNetwork};

use crate::arp_cache::ArpCacheMutex;

pub mod net_arp_listener;
pub mod net_arp_sender;
pub mod net_arp_watcher;

pub struct NetArpWatcher {
    interface: NetworkInterface,
}

pub struct NetArpSender {
    tx: Box<dyn DataLinkSender>,
    source_mac: MacAddr,
    source_ip: Ipv4Addr,
    network_addr: IpNetwork,
}

pub struct NetArpListener {
    rx: Box<dyn DataLinkReceiver>,
    arp_cache: ArpCacheMutex,
}