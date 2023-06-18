
# ARP Watch (TUI)

Watch for ARP cache change and alert user with a notification

**Work in progress**

## Dependencies
only `libnotify` and Rust are required, let me know if you encounter any dependency issue

## Install
Clone the repo and use the bash wrapper for starting the app
```
git clone https://github.com/0xpaco/arp-watch-tui
cd arp-watch-tui
./start_arp_watch.sh
```
> TODO: Makefile clean install and uninstall in path

## Usage

`./start_arp_watch.sh <interface>`

## Known issue
Most users require privilege for sniffing packet on an interface, 
however the sudo privilege doesn't keep environment variable required by dbus.

The `start_arp_watch.sh` provide a temporary fix by starting the program with sudo running dbus-launch.

Feel free to contribute or share your ideas as the current *fix* is temporary

## Features

- [x] Track new or changed ARP entry
- [ ] Daemon 
- [x] Desktop Notification (TODO fix dbus)
- [ ] auto ReARP
- [ ] TUI 
	- [x] Logs
	- [ ] List hosts
	- [ ] ARP Traffic graph 
	- [ ] Options 

