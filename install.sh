cargo build --release
cp paction.service ~/.config/systemd/user/
sudo cp target/release/paction /usr/bin/
