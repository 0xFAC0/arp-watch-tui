use std::{env, sync::Arc};

use arp_watch::{arp_cache::ArpCache, net_arp::*, ui::*};
use tokio::{join, sync::Mutex};

#[tokio::main]
async fn main() {
    // println!("<><><> Arp Watch <><><>");

    let arp_cache = ArpCache::new(false);
    let arp_cache_mutex = Arc::new(Mutex::new(arp_cache));

    let net_watcher = NetArpWatcher::new(env::args().nth(1));
    let mut listener = net_watcher.listener(arp_cache_mutex.clone());
    let sender = net_watcher.sender();

    let app = App::new(arp_cache_mutex, sender);

    join!(tui::main_tui(app), listener.packet_handler());
}
