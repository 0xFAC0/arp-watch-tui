use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{arp_cache::ArpCacheMutex, net_arp::NetArpSender};

use super::App;

impl App {
    pub fn new(arp_cache: ArpCacheMutex, net_sender: NetArpSender) -> Self {
        Self { _arp_cache: arp_cache, net_sender: Arc::new(Mutex::new(net_sender)) }
    }
}