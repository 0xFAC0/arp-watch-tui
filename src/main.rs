use std::{env, sync::Arc};

use arp_watch::{arp_watcher::*, arp_cache::{ArpCache}};
use tokio::{join, sync::Mutex};

#[tokio::main]
async fn main() {
    println!("<><><> Arp Watch <><><>");
    
    let watcher = ArpWatcher::new(env::args().nth(1));
    let arp_cache = ArpCache::new().unwrap();
    let arp_cache_mutex = Arc::new(Mutex::new(arp_cache));
    let mut listener = watcher.listener(arp_cache_mutex);
    let mut sender = watcher.sender();
    join!(sender.scan_network(), listener.packet_handler());
    
}
