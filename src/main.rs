use std::{
    env::args,
    sync::{Arc, Mutex},
    thread,
};
use arp_watch::{arp_cache::*, pnet_sniffer::*};
use notify_rust::Notification;

fn main() {
    println!("<><><> Arp Watch <><><>");

    println!("{:#?}", notify_rust::dbus_stack().unwrap());
    let arpcache_mutex: ArpCacheMutex = Arc::new(Mutex::new(ArpCache::new().unwrap()));
    let arpcache_mutex_cpy = arpcache_mutex;

    let iface_name: Option<String> = args().nth(1);

    let thread = thread::spawn(move || {
        let mut sniffer = Sniffer::new(iface_name, arpcache_mutex_cpy).unwrap();
        sniffer.scan_network();
        sniffer.rx_th.join().unwrap();
    });

    Notification::new()
        .appname("Arp watch alert")
        .summary("Monitoring arp traffic")
        .show()
        .unwrap();

    thread.join().unwrap();
}
