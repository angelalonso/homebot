# How is this architected?

## NOTE on our running environments:
We have three running environments:
- sim, where the code is run on a simulation environment using Webots.
  - The challenge is that everything is depending on some bindings between Webots and the Rust language.
- test, where the code is run (usually for tests) on the laptop.
  - The challenge here is that the GPIO is not expected to work, but we still want to test some other stuff.
- live, where the code is run on the robot itself.  
  
So, if we have a file called env.rs and another one called sim_env.rs, the first one will be used on test and live and the second one, on the sim environment.

## Logic path
  
main.rs is the entry point for everyone, which then calls the run function on either sim_env, or env.  
ToBeContinued...
