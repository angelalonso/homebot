# Complete suite to control my own Home robot

The goal is to have a system to control my home robot.
My main challenge so far as been managing complex sets of actions, and so this time I will try to follow these rules:
- Start small, with simple action sets.
- The code should run fast but also should be easy to understand and expand.
- Everything should be tested and simulated.
- The simulation should correlate with what happens at the robot.

# TL;DR
1. Download and install [Webots](https://cyberbotics.com)
1. Install [Rust](https://www.rust-lang.org/learn/get-started)
1. Run 
  1. ./home.sh test for the code tests
  1. ./home.sh sim for the simulated mode
  1. ./home.sh install to install into the actual robot
  1. ./home.sh live to control how it runs in the actual robot


# Robot Software installation
See [the related Readme for the installation on the robot](ROBOT_README.md)


# How the code under src/ is architected
See [ARCHITECTURE.md](ARCHITECTURE.md)

# What works
- Simulation mode works
- Added a home.sh script to trigger test, sim (or live in the future) modes
- Modified current workflow for Webots, including everything needed to run the code on simulation mode with ./home.sh sim
- Separated code that is exclusive to one mode or another (e.g.: sim needs to call webots bindings, test does not)
- Have tests for the functionality, like a grown-up program!

# What does not work yet
Installation into robot
See TODO.txt

## Why?
### Why is it so tightly linked to Webots?
I had a working version in https://github.com/angelalonso/robot but it had several problems and the only way to improve was to have a testing environment.

For this new project I based it originally on https://github.com/acj/webots-rs as of September 2023. 

The idea was to use that example and build a testing environment, from which I can tailor the code to what my robot actually has (some sensors, some motors...).

### Why using this Stack when there is ROS and Gazebo?
Gazebo seems to be tight friends with ROS. Python on ROS is too slow, I don't know C and using some bindings there was a mess. 
I am trying Webots now because it seems to have a much simpler way of making Rust work.



# Original docs for how the webots part works:

1. Download and install [Webots](https://cyberbotics.com) for your operating system
1. Install [Rust](https://www.rust-lang.org/learn/get-started) if you haven't already
1. Clone this repository
1. Run `make` to compile the Rust controller and copy it into place
1. Open the `sample_project/worlds/my_first_simulation.wbt` file in Webots
1. Run the simulation

You should see "The Rust controller has started" in the Webots console.

To make changes to the controller, you can edit `src/main.rs` and then run `make` again. You might need to reset the simulation (File > Reset Simulation) or restart Webots to use the updated code.

### How this works

At compile time, we use [bindgen](https://github.com/rust-lang/rust-bindgen) to convert a list of Webots C header files (see `wrapper.h`) into Rust structures and types. Those types form a bridge between the Rust-based controller code and the Webots C library that does the hard work of interacting with the simulation engine. See `build.rs` for more details.

## License

MIT
Please find The original LICENSE from Adam Jensen under LICENSE_included. Any further works from that day on are also licensed under the MIT one.


