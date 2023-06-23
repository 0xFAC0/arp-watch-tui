use crate::{
    arp_cache::{ArpCacheMutex, ArpEntry},
    net_arp::NetArpSenderMutex,
};

pub mod app;
pub mod arp_cache_widget;
pub mod helper;
pub mod tui;

pub struct App {
    arp_cache: ArpCacheMutex,
    net_sender: NetArpSenderMutex,
}

pub struct UiSettings {
    arp_entries: Vec<ArpEntry>,
    follow_mode: bool,
}
