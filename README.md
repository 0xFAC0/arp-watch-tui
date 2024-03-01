
# ARP Watch (TUI)

Watch for ARP cache change and alert user with a notification

This project is no longer maintained. 

## Dependencies
only `libnotify` and Rust are required, let me know if you encounter any dependency issue

## Install
Clone the repo and use the bash wrapper for starting the app
```
git clone https://github.com/0xpaco/arp-watch-tui
cd arp-watch-tui
cargo build --release
```
> TODO: Makefile clean install and uninstall in path

## Usage

`sudo -E ./target/release/arp-watch-tui`

## Known issue
Most users require privilege for sniffing packet on an interface, 
however the sudo privilege doesn't keep environment variable required by dbus.

You might need to use `sudo -E` to circumvent this.

Feel free to contribute or share your ideas as the current *fix* is temporary

## Features

- [x] Track new or changed ARP entry
- [ ] Daemon 
- [x] Desktop Notification (TODO fix dbus)
- [ ] Kernel module
- [ ] TUI 
	- [x] Logs
	- [ ] List hosts
	- [ ] ARP Traffic graph 
	- [ ] Options 

