# paction
Execute actions relative to process existance.

## Configuration
The configuration may contain multiple actions, each with multiple or singular criteria. Default location is `~/.config/paction/config.toml`, see `paction --help` for more options. Copy the [sample config](config.sample.toml) for reference when configuring.


### Example config
Stop `compton` when either **CS:GO** or **THUG Pro** *and* **OBS Studio** is running - to get those extra frames you know. Once criteria is no longer met, start compton again.

```toml
tick = 500

[[action]]
name = "some action" # Name used in output
exec = ["pkill", "compton"]
undo = ["compton"]

# CS:GO or THUG Pro must be running by the same user as paction
[[action.criteria]]
user = ["$USER"]
name = ["csgo_linux64", "THUGPro.exe"]
cmd = []

# OBS Studio must ALSO be running
# By either "someuser", 1001 or the same user as paction
# With the switch --startreplaybuffer
[[action.criteria]]
user = ["someuser", 1001, "$USER"]
name = ["obs"]
cmd = ["--startreplaybuffer"]
```

### Running as service
For X display support (e.g. start a GUI program on `exec`) I recommend running as a user service. Note however that the service may need to be modified to start after your display manager, otherwise certain actions/scripts requiring $DISPLAY might not work when the service is automatically started. 

Personally I have no valid `After=` target for the service because I don't use a display manager, so I just add `systemctl --user start paction` in `.xinitrc` and omit enabling the service.
