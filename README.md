# Complete suite to control my own Home robot

The goal is to have a system to control my home robot.
Some kind of wrapper to define actions would also be great to have.

# How the code under src/ is architected
- Main - checks mode and triggers different libraries depending on whether it runs on a laptop (sim) or the robot (live)
- Both sim and live should be structured as similar as it gets
  - Each library that is EXCLUSIVELY for one or the other starts with sim_ or live_ (e.g.: sim_bindings.rs)
  - A library can start as exclusive to one (e.g.: sim_brain.rs) and then be renamed once adapted for the other (then becoming just brain.rs)
- test_env.rs, sim_env.rs and live_env.rs each will have its own run function.
  - This function first initializes what each mode requires then run a loop, but both modes should follow a similar structure
- Regarding that loop, it uses the following mechanisms
  - Time - To base changes on time passed since the program started -  shall we treat time as an input maybe?
  - Input - This gets the current values from several sensors
    - For now we don't turn those sensors on or off, we just ignore values that we don't need
  - Brain - This "analyzes" the Input (Time also being one) schedules actions to be taken

## What is the plan now?
I had a working version in https://github.com/angelalonso/robot but it had several problems and the only way to improve was to have a tesitng environment.

Gazebo seems to be tight friends with ROS. Python on ROS is too slow, I don"t know C and using some bindings there was a mess. 
I am trying Webots now because it seems to have a much simpler way of making Rust work.

This project is based on https://github.com/acj/webots-rs as of September 2023. 

The idea was to use that example and build a testing environment, from which I can tailor the code to what my robot actually has (some sensors, some motors...).

Now that I have a base, I will create another mode for when the code runs live on the actual robot instead of a Webots simulation or a test.

## DONE :
- Added a home.sh script to trigger test, sim or live modes
- Modified current workflow for Webots, including everything needed to run the code on simulation mode with ./home.sh sim
- Separated code that is exclusive to one mode or another (e.g.: sim needs to call webots bindings, test does not)
- Have tests for the functionality, like a grown-up program!

## NEXTUP:

See TODO.txt

## Original docs for how the webots part works:

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
