use tokio::task::JoinHandle;

use crate::arp_cache::ArpCacheMutex;

use super::*;

impl App {
    pub fn new(arp_cache: ArpCacheMutex) -> Self {
        Self { arp_cache }
    }

    pub async fn toggle_follow_mode(&mut self) {
        let mut arp_cache = self.arp_cache.lock().await;
        arp_cache.follow_update = !arp_cache.follow_update;
    }

    pub async fn toggle_enable_rearping(&mut self) {
        let mut arp_cache = self.arp_cache.lock().await;
        arp_cache.rearp_enable = !arp_cache.rearp_enable;
    }

    pub async fn get_ui_settings(&self) -> UiSettings {
        let arp_cache = self.arp_cache.lock().await;
        UiSettings {
            arp_entries: arp_cache.entries(),
            follow_mode: arp_cache.follow_update,
            rearp_enable: arp_cache.rearp_enable
        }
    }

    pub async fn start_scan_hosts(&self) -> JoinHandle<()> {
        let arp_cache_mutex = self.arp_cache.clone();
        tokio::spawn(async move {
            let mut arp_cache = arp_cache_mutex.lock().await;
            arp_cache.start_network_scan();
        })
    }
}
