# Complete suite to control my own Home robot

The goal is to have a code base to control my home robot.

## What is the plan now?
I had a working version in https://github.com/angelalonso/robot but it had several problems and the only way to improve was to have a tesitng environment.

Gazebo seems to be tight friends with ROS. Python on ROS is too slow, I don"t know C and using some bindings there was a mess. 
I am trying Webots now because it seems to have a much simpler way of making Rust work.

This project is based on https://github.com/acj/webots-rs as of September 2023. 

I am planning on using that example to have a testing environment from which I can tailor the code to what my robot actually has (some sensors, some motors...).

Once that works, I will create a second mode for the robot, for when the code runs live on the actual robot instead of a Webots simulation.

## Running the Testing simulation

1. Download and install [Webots](https://cyberbotics.com) for your operating system
1. Install [Rust](https://www.rust-lang.org/learn/get-started) if you haven't already
1. Clone this repository
1. Run `make` to compile the Rust controller and copy it into place
1. Open the `sample_project/worlds/my_first_simulation.wbt` file in Webots
1. Run the simulation

You should see "The Rust controller has started" in the Webots console.

To make changes to the controller, you can edit `src/main.rs` and then run `make` again. You might need to reset the simulation (File > Reset Simulation) or restart Webots to use the updated code.

## How this works

At compile time, we use [bindgen](https://github.com/rust-lang/rust-bindgen) to convert a list of Webots C header files (see `wrapper.h`) into Rust structures and types. Those types form a bridge between the Rust-based controller code and the Webots C library that does the hard work of interacting with the simulation engine. See `build.rs` for more details.

## License

MIT
Please find The original LICENSE from Adam Jensen under LICENSE_included. Any further works from that day on are also licensed under the MIT one.
