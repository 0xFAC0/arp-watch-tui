use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    arp_cache::{ArpCacheMutex, ArpEntry},
    net_arp::NetArpSender,
};

use super::App;

impl App {
    pub fn new(arp_cache: ArpCacheMutex, net_sender: NetArpSender) -> Self {
        Self {
            arp_cache,
            net_sender: Arc::new(Mutex::new(net_sender)),
        }
    }

    pub async fn arp_entries(&self) -> Vec<ArpEntry> {
        let arp_cache = self.arp_cache.lock().await;
        arp_cache.entries()
    }

    pub async fn toggle_follow_mode(&mut self) {
        let mut arp_cache = self.arp_cache.lock().await;
        arp_cache.follow_update = !arp_cache.follow_update;
    }
}
