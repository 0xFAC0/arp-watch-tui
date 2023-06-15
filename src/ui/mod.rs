use crate::{arp_cache::ArpCacheMutex, net_arp::NetArpSenderMutex};

pub mod app;
pub mod tui;

pub struct App {
    arp_cache: ArpCacheMutex,
    net_sender: NetArpSenderMutex,
}
