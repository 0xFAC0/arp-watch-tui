use crate::{arp_cache::ArpCacheMutex, net_arp::NetArpSenderMutex};

pub mod app;
pub mod tui;

pub struct App {
    _arp_cache: ArpCacheMutex,
    net_sender: NetArpSenderMutex,
}
