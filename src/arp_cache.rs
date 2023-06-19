use std::{net::Ipv4Addr, path::Path, sync::Arc};
use tokio::{fs::File, io::AsyncReadExt};

use log::{info, warn};
use pnet::util::MacAddr;
use tokio::sync::Mutex;

use crate::{alert::alert, net_arp::NetArpSender};

const PATH: &str = "/proc/net/arp";

pub type ArpCacheMutex = Arc<Mutex<ArpCache>>;

#[derive(Default)]
pub struct ArpCache {
    vec: Vec<ArpEntry>,
    pub follow_update: bool,
    pub rearp_enable: bool,
    net_sender: Option<NetArpSender>,
}

pub enum ArpCacheUpdateResult {
    NewEntry,
    AlreadyExist,
    EntryDiff,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ArpEntry {
    ip: Ipv4Addr,
    mac: MacAddr,
}

impl ArpCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn follow_update(mut self, follow_update: bool) -> Self {
        self.follow_update = follow_update;
        self
    }

    pub fn rearp(mut self, rearp: bool) -> Self {
        self.rearp_enable = rearp;
        self
    }

    pub fn net_sender(mut self, net_sender: NetArpSender) -> Self {
        self.net_sender = Some(net_sender);
        self
    }

    pub async fn parse_from_cache(mut self) -> Self {
        self.vec = Self::parse_fs_cache().await;
        self
    }

    pub fn entries(&self) -> Vec<ArpEntry> {
        self.vec.clone()
    }

    pub fn start_rearping(&mut self, entry: ArpEntry) {
        match &mut self.net_sender {
            Some(net_sender) => net_sender.rearp(entry),
            None => warn!("Could not rearp {:?}, no sender available", entry),
        };
    }

    pub fn start_network_scan(&mut self) {
        match &mut self.net_sender {
            Some(net_sender) => {
                net_sender.scan_network().unwrap();
            }
            None => warn!("Could not start the scan, no sender available"),
        }
    }

    async fn parse_fs_cache() -> Vec<ArpEntry> {
        let mut vec = vec![];

        let path = Path::new(PATH);
        let mut file = File::open(path).await.unwrap();

        let mut file_content = String::new();
        file.read_to_string(&mut file_content).await.unwrap();
        for line in file_content.lines() {
            let mut words = line.split_whitespace();
            let ip_str = match words.next() {
                Some(addr) => {
                    if addr == "IP" {
                        continue;
                    }
                    addr
                }
                None => continue,
            };
            words.next();
            words.next();
            // TODO parse the last collumn for the mask
            let mac_str = words.next().unwrap();
            info!(
                "Making new ARP Entry from existing cache: {} {}",
                ip_str, mac_str
            );
            let new_entry = ArpEntry::from(ip_str, mac_str);

            vec.push(new_entry);
        }
        vec
    }

    pub fn update(&mut self, new_entry: ArpEntry) -> ArpCacheUpdateResult {
        let mut entry_diff: Option<(usize, ArpEntry)> = None;
        for (i, entry) in self.vec.iter().enumerate() {
            if new_entry.ip == entry.ip && new_entry.mac == entry.mac {
                warn!("Entry already exist");
                return ArpCacheUpdateResult::AlreadyExist;
            }
            if entry.ip == new_entry.ip && entry.mac != new_entry.mac {
                warn!("Entry divergence spotted");
                entry_diff = Some((i, entry.clone()));
                alert(format!(
                    "[{}]\nwas {}, now {}",
                    entry.ip, entry.mac, new_entry.mac
                ));
            }
        }

        match entry_diff {
            Some((i, old_entry)) => {
                if self.follow_update {
                    self.vec.push(new_entry);
                    self.vec.remove(i);
                }

                if self.rearp_enable {
                    match &mut self.net_sender {
                        Some(net_sender) => net_sender.rearp(old_entry),
                        None => (),
                    }
                }
                ArpCacheUpdateResult::EntryDiff
            }
            None => {
                alert(format!("{} at {}", new_entry.ip, new_entry.mac));
                self.vec.push(new_entry);
                warn!("New entry registered");

                return ArpCacheUpdateResult::NewEntry;
            }
        }
    }
}

impl ArpEntry {
    pub fn new(ip: Ipv4Addr, mac: MacAddr) -> Self {
        Self { ip, mac }
    }

    pub fn from(ip_str: &str, mac_str: &str) -> Self {
        let ip: Ipv4Addr = ip_str.parse().unwrap();
        let mac: MacAddr = mac_str.parse().unwrap();
        Self { ip, mac }
    }

    pub fn ip(&self) -> &Ipv4Addr {
        &self.ip
    }

    pub fn mac(&self) -> &MacAddr {
        &self.mac
    }
}
