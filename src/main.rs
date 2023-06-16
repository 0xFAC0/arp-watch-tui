use std::{env, sync::Arc};

use arp_watch::{arp_cache::ArpCache, net_arp::*, ui::*};
use log::error;
use tokio::{select, sync::Mutex};

#[tokio::main]
async fn main() {
    tui_logger::init_logger(log::LevelFilter::Trace).unwrap();
    let arp_cache = ArpCache::new(false);
    let arp_cache_mutex = Arc::new(Mutex::new(arp_cache));

    let net_watcher = NetArpWatcher::new(env::args().nth(1));
    let mut listener = net_watcher.listener(arp_cache_mutex.clone());
    let sender = net_watcher.sender();

    let app = App::new(arp_cache_mutex, sender);

    select!(
        ret = tui::main_tui(app) => {
            if let Err(e) = ret {
                error!("TUI Failed: {e}")
            }
        },
        ret = listener.packet_handler() => {
            if let Err(e) = ret {
                error!("Packet handler failed: {e}")
            }
        },
    );
}
