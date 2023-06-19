use std::{net::Ipv4Addr, sync::Arc};

use pnet::{
    datalink::{DataLinkReceiver, DataLinkSender, NetworkInterface},
    ipnetwork::IpNetwork,
    util::MacAddr,
};
use tokio::sync::Mutex;

use crate::arp_cache::ArpCacheMutex;

pub mod net_arp_listener;
pub mod net_arp_sender;
pub mod net_arp_watcher;

pub type NetArpSenderMutex = Arc<Mutex<NetArpSender>>;

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
