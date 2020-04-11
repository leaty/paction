# paction
Execute actions relative to process existance

## Configuration
The configuration may contain multiple actions, each with multiple or singular criteria. Default location is `~/.config/paction/config.toml` but see `paction --help` for more. Copy the [sample config](config.sample.toml) for reference when configuring.


### Example config
Stop compton when both `CS:GO` and `OBS Studio` is running. Once criteria is no longer met, start compton again.

```toml
tick = 100 # Updates per millisecond

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
