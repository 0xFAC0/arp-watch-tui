if [[ ! -e ./target/release/arp-watch ]]; then
    cargo build -r --no-default-features
fi
sudo dbus-launch --exit-with-session ./target/release/arp-watch $@