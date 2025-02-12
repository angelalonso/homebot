# TL;DR

Installation on the robot is done at ./robot_install (after you put together the hardware and install a base OS on the microSD)

To control the Robot, head to ./homebotctl

In both cases, please check the README.md first

# Status
There are three parts to this software:
- robot_install
  - Installs requirements on Raspberry Pi.
  - 75% working, needs review and tests
- homebotctl
  - Controls how and what runs on the robot.
  - 50% of the basic functionality working, an advanced set of functions will follow.
- homebot
  - This is the robot software itself.
  - The first phase of basic movements works on Simulation but not yet on Hardware. Calibration phase should be next.
