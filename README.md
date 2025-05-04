# TL;DR

I want to put together a robot that is flexible enough to not need constant input to do tasks.

Since I have no professional background on robotics, I need to progress step by step until I get a good-enough system.

This README is basically a mapping of those steps and a record of the progress.

# Bare minimum Goal

I leave the robot on the floor and it can automagically move around without hitting obstacles

## Derived Requirements
- The Robot moves
  - It has wheels
  - It has motors to move those wheels
  - It has a Motor controller (L298N)
  - It has batteries for that Motor controller
  - It has a Raspberry Pi (Raspi) that manages the L298N
    - The Raspi has a battery
    - The Raspi boots an OS
    - The Raspi can turn the wheels on and off
      - (Non-derived req) The Raspi can turn wheels on and off from the command line
      - The Raspi has a program (homebot) that can make the L298N move the Motors
        - (Non-derived req) homebot is written in Rust
        - (Non-derived req) there is a way to install homebot
          - homebot can be installed from my laptop
            - my laptop can SSH into the Raspi
            - my laptop can cross-compile for the Raspi
          - homebot can be tested locally on my laptop
          - homebot can be simulated locally on my laptop
- The robot is independent on how and when to move
  - TBD
