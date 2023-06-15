use std::{error::Error, fs::File, io::Read, net::Ipv4Addr, path::Path, sync::Arc};

use pnet::util::MacAddr;
use tokio::sync::Mutex;

use crate::alert::alert;

const PATH: &str = "/proc/net/arp";

pub type ArpCacheMutex = Arc<Mutex<ArpCache>>;

#[derive(Debug, Clone)]
pub struct ArpCache {
    vec: Vec<ArpEntry>,
    follow_update: bool,
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
    pub fn new(follow_update: bool) -> Self {
        let mut ret = ArpCache {
            vec: vec![],
            follow_update,
        };
        ret.parse().unwrap();
        ret
    }

    pub fn parse(&mut self) -> std::result::Result<u8, Box<dyn Error>> {
        let mut entry_count: u8 = 0;

        let path = Path::new(PATH);
        let mut file = File::open(path)?;

        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();
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
            let new_entry = ArpEntry::from(ip_str, mac_str);

            self.vec.push(new_entry);
            entry_count += 1;
        }
        Ok(entry_count)
    }

    pub fn update(&mut self, new_entry: ArpEntry) -> ArpCacheUpdateResult {
        let mut entry_diff = false;
        for entry in self.vec.iter() {
            if new_entry.ip == entry.ip && new_entry.mac == entry.mac {
                // println!("[ARP Cache] Entry already exist");
                return ArpCacheUpdateResult::AlreadyExist;
            }
            if entry.ip == new_entry.ip && entry.mac != new_entry.mac {
                // println!("[ARP Cache] Entry divergence spotted");
                entry_diff = true;
                alert(format!(
                    "[{}]\nwas {}, now {}",
                    entry.ip, entry.mac, new_entry.mac
                ));
            }
        }

        if !entry_diff {
            alert(format!("{} at {}", new_entry.ip, new_entry.mac));
            if self.follow_update {
                self.vec.push(new_entry);
            }
            //println!("[ARP Cache] New entry registered");

            return ArpCacheUpdateResult::NewEntry;
        }
        ArpCacheUpdateResult::EntryDiff
    }
}

impl ArpEntry {
    pub fn new(ip: Ipv4Addr, mac: MacAddr) -> Self {
        Self { ip, mac }
    }

    pub fn from(ip_str: &str, mac_str: &str) -> Self {
        // println!(
        //     "Making new ARP Entry from existing cache: {} {}",
        //     ip_str, mac_str
        // );
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
