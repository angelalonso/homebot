# Modes

## Test Mode
./cargo run test

- Cargo test the code

## Sim Mode
./cargo run sim

- Runs the code on Webots

## Build Mode
./cargo run build

- Run test Mode
- Compile for Raspberry arch

## Deploy Mode
- TBD: stop systemd if running
- scp new binary and systemd
- TBD: transfer config files to raspi
- TBD: validate config files
  - set up dependencies and env variables
  - Security settings (Change passwords or keys)
  - network configuration
  - TODO: ctl.cfg is a separate config file
  - TODO: Function to run commands over SSH
- TBD: start systemd and captures output


# TO DO List
This is a list of what modes of the robot this program should handle

# Main functions
----------------------------
These functions are the base to make the robot work

## Stop Mode
Send stop commands to all actuators
Disable sensors
Shut down program
Init 0 the raspi

# Advanced functions
----------------------------
These functions will be added after the Main ones are ready

## Sensor Calibration Mode
run calibration routines
save calibration data
validate calibration results

## Localization and Mapping Mode
Start mapping routines
Save or update maps
Visualize map and robot position

## User Interface Mode
start a web server or terminal UI
provide options for controlling robot
display logs and status

## Detached Mode
Runs Deploy Mode but does not capture output

## Health Check Mode
run diagnostics on robot
  monitor battery levels
report status of various components
send summary
  this one only makes sense after running on detached mode

## Installation Mode
Runs Build Mode
set up systemd (until then, this is done once by hand)
Runs Deploy Mode

## Recovery Mode
Runs Deploy Mode

## Re-Configuration Mode
transfer config files to raspi
validate config files
  set up dependencies and env variables
  Security settings (Change passwords or keys)
  network configuration
TODO: Program should be able to read new configs on the fly

## Remote Debugging Mode
start program with debugging enabled
setup remote debugging sessions
Stream logs back to laptop

## Data Collection Mode
transfer past data to laptop
For current data/logs, the remote debugging mode should do

## Task Scheduling Mode
Set up task schedules
Monitor and execute scheduled tasks
Log task execution
Tasks could be a configuration that can be modified on the fly


