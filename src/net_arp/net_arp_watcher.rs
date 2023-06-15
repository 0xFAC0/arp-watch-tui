use pnet::datalink::{self, NetworkInterface};

use crate::{
    arp_cache::ArpCacheMutex, 
    net_arp::net_arp_sender::NetArpSender,
};

use super::*;

impl NetArpWatcher {
    pub fn new(name: Option<String>) -> Self {
        if let Some(name) = name {
            match datalink::interfaces()
                .into_iter()
                .find(|interface: &NetworkInterface| interface.name == name)
            {
                Some(interface) => Self { interface },
                None => panic!("No interface found"),
            }
        } else {
            for interface in datalink::interfaces().into_iter() {
                if !interface.is_loopback() {
                    return Self { interface };
                }
            }
            panic!("No avaible network interface found");
        }
    }

    pub fn listener(&self, arp_cache: ArpCacheMutex) -> NetArpListener {
        NetArpListener::new(&self.interface, arp_cache)
    }

    pub fn sender(&self) -> NetArpSender {
        NetArpSender::new(&self.interface)
    }
}
