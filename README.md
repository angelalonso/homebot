# TL;DR

I want to put together a robot that is flexible enough to not need constant input to do tasks.

Since I have no professional background on robotics, I need to progress step by step until I get a good-enough system.

This README is basically a mapping of those steps and a record of the progress.

<<<<<<< HEAD
# Progress
Following tasks are considered done when they are also documented:
- Robot's Hardware
TBD - Put together a Chassis for the robot
TBD - Connect at least Raspberry - Controller - Motors - related Batteries
    - Connect Raspberry to Router
    - Install OS
    - OS minimal configuration
    - OS configuration: GPIO usable
      -  Python test: see 01_pythontest Folder
    - Service file
    - ...
- Laptop's Software
    - Cross-Build homebot locally
    - Test
    - Deploy
    - Start Service
    - Stop Service
    - See output of running Service
    - ...
- Robot's Software
    - Run Motors
    - Run Motors, stop, Run back
    - ...
=======
# Bare minimum Goal
>>>>>>> 3b98c3379a47e37677c9d54372678b81a3a161a5

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
      - OK - (Non-derived req) The Raspo can control GPIO from a Python script - See 01_pythontest
      - (Non-derived req) The Raspo can control GPIO from a Rust script
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
