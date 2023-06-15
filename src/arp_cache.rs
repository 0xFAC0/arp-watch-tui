use notify_rust::Notification;
use pnet::util::MacAddr;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::net::{AddrParseError, Ipv4Addr};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::vec;

type Result<T> = std::result::Result<T, ArpCacheErrors>;
pub type ArpCacheMutex = Arc<Mutex<ArpCache>>;

const CACHE_STR_PATH: &str = "/proc/net/arp";

#[derive(Debug, Clone)]
pub struct ArpCache {
    vec: Vec<ArpEntry>,
}

impl ArpCache {
    pub fn new() -> Result<Self> {
        let mut ret = ArpCache {
            vec: vec![],
        };
        ret.fetch()?;
        Ok(ret)
    }

    pub fn fetch(&mut self) -> Result<u8> {
        let mut entry_count: u8 = 0;

        let path = Path::new(CACHE_STR_PATH);
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(io_err) => return Err(ArpCacheErrors::IOError(io_err)),
        };

        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;
        for line in file_content.lines() {
            let mut words = line.split_whitespace();
            let first_column = words.next();
            if first_column.is_none() || first_column.unwrap() == "IP" {
                continue;
            }

            words.next();
            words.next();
            // TODO parse the last collumn for the mask
            let ip_str = first_column.unwrap();
            let mac_str = words.next().unwrap();
            let new_entry = match ArpEntry::from(ip_str, mac_str) {
                Ok(entry) => entry,
                Err(err) => return Err(err),
            };
            self.vec.push(new_entry);
            entry_count += 1;
        }
        Ok(entry_count)
    }

    pub fn update(&mut self, new_entry: ArpEntry) -> ArpCacheUpdateResult {
        let mut entry_diff = false;
        for entry in self.vec.iter() {
            if new_entry.ip == entry.ip && new_entry.mac == entry.mac {
                println!("[ARP Cache] Entry already exist");
                return ArpCacheUpdateResult::AlreadyExist;
            }
            if entry.ip == new_entry.ip && entry.mac != new_entry.mac {
                println!("[ARP Cache] Entry divergeance spotted");
                entry_diff = true;
                match Notification::new()
                    .appname("Arp watch alert")
                    .summary("Arp entry change")
                    .body(format!("[{}]\nwas {}, now {}", entry.ip, entry.mac, new_entry.mac).as_str())
                    .show() {
                        Ok(_) => (),
                        Err(e) => println!("Notification failed: {e}")
                    };
            }
        }

        if !entry_diff {
            match Notification::new()
                .appname("Arp watch alert")
                .summary("New ARP entry")
                .body(format!("{} at {}", new_entry.ip, new_entry.mac).as_str())
                .show() {
                    Ok(_) => (),
                    Err(e) => println!("Notification failed: {e}")
                };
            self.vec.push(new_entry);
            println!("[ARP Cache] New entry registered");
            return ArpCacheUpdateResult::NewEntry;
        }
        return ArpCacheUpdateResult::EntryDiff;
    }
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

impl ArpEntry {
    pub fn new(ip: Ipv4Addr, mac: MacAddr) -> Self {
        Self { ip, mac }
    }

    pub fn from(ip_str: &str, mac_str: &str) -> Result<Self> {
        println!(
            "Making new ARP Entry from existing cache: {} {}",
            ip_str, mac_str
        );
        let ip: Ipv4Addr = match ip_str.parse() {
            Ok(ip) => ip,
            Err(err) => {
                return Err(ArpCacheErrors::IpParserError(err));
            }
        };
        let mac: MacAddr = match mac_str.parse() {
            Ok(mac) => mac,
            Err(_) => {
                return Err(ArpCacheErrors::MacParserError);
            }
        };
        Ok(Self { ip, mac })
    }

    pub fn ip(&self) -> &Ipv4Addr {
        &self.ip
    }

    pub fn mac(&self) -> &MacAddr {
        &self.mac
    }
}

#[derive(Debug)]
pub enum ArpCacheErrors {
    IOError(std::io::Error),
    IpParserError(AddrParseError),
    MacParserError,
}

impl From<std::io::Error> for ArpCacheErrors {
    fn from(value: std::io::Error) -> Self {
        ArpCacheErrors::IOError(value)
    }
}

impl Display for ArpCacheErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArpCacheErrors::IOError(io_err) => write!(f, "Arp cache IO Error: {}", io_err),
            ArpCacheErrors::IpParserError(parser_err) => {
                write!(f, "Failed to parse ip from arp cache: {}", parser_err)
            }
            ArpCacheErrors::MacParserError => {
                write!(f, "Failed to parse mac from arp cache")
            }
        }
    }
}

impl Error for ArpCacheErrors {}
