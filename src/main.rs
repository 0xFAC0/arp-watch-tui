use std::{env, sync::Arc};

use arp_watch::{net_arp::*, arp_cache::{ArpCache}};
use tokio::{join, sync::Mutex};

#[tokio::main]
async fn main() {
    println!("<><><> Arp Watch <><><>");
    
    // TODO
    // ArpWatcher -> NetArpWatcher -> Only log ARP change payloads in traffic
    // ArpCache -> represent the cache
    //          -> update: bool
    // ArpCacheWatcher -> Periodicly watch for arp cache file (? use notify)
    //                 -> In charge of the alert
    //                 -> Prevent change if !ArpCache.update
    let watcher = NetArpWatcher::new(env::args().nth(1));
    let arp_cache = ArpCache::new().unwrap();
    let arp_cache_mutex = Arc::new(Mutex::new(arp_cache));
    let mut listener = watcher.listener(arp_cache_mutex);
    let mut sender = watcher.sender();
    join!(sender.scan_network(), listener.packet_handler());
    
}
