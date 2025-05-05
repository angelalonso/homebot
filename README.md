# TL;DR

I want to put together a robot that is flexible enough to not need constant input to do tasks.

Since I have no professional background on robotics, I need to progress step by step until I get a good-enough system.

This README is basically a mapping of those steps and a record of the progress.

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








Installation on the robot is done at ./robot_install (after you put together the hardware and install a base OS on the microSD)

To control the Robot, head to ./homebotctl

In both cases, please check the README.md first

# Status
There are three parts to this software:
- robot_install
  - Installs requirements on Raspberry Pi (not the homebot program itself)
  - Needs review and tests
- homebotctl
  - Controls how and what runs on the robot and how it is installed/updated.
  - 50% of the basic functionality working, an advanced set of functions will follow.
  - Details: [README.md](./homebotctl/README.md)
- homebot
  - This is the robot software itself.
  - The first phase of basic movements works on Simulation but not yet on Hardware. Calibration phase should be next.
  - A README specific to this part is not yet ready.
    - It will be done once robot_install and homebotctl are working (at least on an MVP state)
