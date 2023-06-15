# ARP Watch (TUI)

Watch for ARP cache change and alert user with a notification
The terminal user interface is still in dev and has been removed temporary  for polishing the back-end
I will update the repo with the TUI made with `tui-rs` as soon as I'm satisfied with the back-end state
and when I will have the dbus issue fixed

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

