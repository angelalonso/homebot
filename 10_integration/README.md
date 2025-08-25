# How is this architected?

## NOTE on our running environments:
We have three running environments:
- sim, where the code is run on a simulation environment using Webots.
  - Everything depends on some bindings between Webots and the Rust language. That makes the code different from live.
- test, where the code is run on the laptop (usually for tests).
  - The challenge here is that the GPIO is not expected to work, but we still want to test some other stuff.
- live, where the code is run on the robot itself.  
  
The file names indicate their environment: if we have a file called env.rs and another one called env_sim.rs, the first one will be used on test and live and the second one, on the sim environment.

## Logic path
  
- main.rs is the entry point for everyone, which then calls the run function.
  - the run function initializes the robot and runs a loop that asks brain to update
    - brain gets data from input and builds a state
      - That data comes from:
        - sensors attached to the arduino
        - timestamp, both the initial one and the elapsed
        - eventually we will store here the state of other stuff included motors
    - with that state it calculates actions to take
      - These can be actions to take immediately, after some time, or after the current action is done
    - brain then sends the actions to output
