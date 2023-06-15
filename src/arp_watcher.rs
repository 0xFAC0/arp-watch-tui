use pnet::datalink::{self, NetworkInterface};

use crate::{arp_listener::ArpListener, arp_cache::ArpCacheMutex, arp_sender::ArpSender};

pub struct ArpWatcher {
    interface: NetworkInterface,
}

impl ArpWatcher {
    pub fn new(name: Option<String>) -> Self {
        if let Some(name) = name {
            match datalink::interfaces().into_iter().find(|interface: &NetworkInterface| interface.name == name) {
                Some(interface) => Self {interface},
                None => panic!("No interface found")
            }
        } else {
            for interface in datalink::interfaces().into_iter() {
                if !interface.is_loopback() {
                    return Self {interface};
                }
            }
            panic!("No avaible network interface found");
        }
    }

    pub fn listener(&self, arp_cache: ArpCacheMutex) -> ArpListener {
        ArpListener::new(&self.interface, arp_cache)
    }

    pub fn sender(&self) -> ArpSender {
        ArpSender::new(&self.interface)
    }
}