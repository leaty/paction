# paction
Execute actions relative to process existance. I really ought to find a better name for this though.

## Configuration
The configuration may contain multiple actions, each with multiple or singular criteria. Default location is `~/.config/paction/config.toml`, see `paction --help` for more options. Copy the [sample config](config.sample.toml) for reference when configuring.


### Example config
Stop `compton` when both **CS:GO** and **OBS Studio** is running - to get those extra frames you know. Once criteria is no longer met, start compton again.

```toml
tick = 100

[[action]]
name = "some action" # Name used in output
exec = ["pkill", "compton"]
undo = ["compton"]

# CS:GO must be running by user 1000
[[action.criteria]]
user = [1000]
name = ["csgo_linux64"]
cmd = []

# OBS Studio must ALSO be running by user 1000, with the switch --startreplaybuffer
[[action.criteria]]
user = [1000]
name = ["obs"]
cmd = ["--startreplaybuffer"]
```

### Running as service
For X display support (e.g. start a GUI program on `exec`) I recommend running as a user service.

You could run `install.sh` or just follow the steps below.

#### Install as user service
```bash
cargo build --release
cp paction.service ~/.config/systemd/user/
sudo cp target/release/paction /usr/bin
systemctl --user enable paction
systemctl --user start paction
```
