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




Raspberry Pi's Model B

                  +3V3---1-|O O|--2--+5V
          (SDA)  GPIO2---3-|O O|--4--+5V
         (SCL1)  GPIO3---5-|O O|--6--_
    (GPIO_GLCK)  GPIO4---7-|O O|--8-----GPIO14 (TXD0)
                       --9-|O.O|-10-----GPIO15 (RXD0)
    (GPIO_GEN0) GPIO17--11-|O O|-12-----GPIO18 (GPIO_GEN1)
    (GPIO_GEN2) GPIO27--13-|O O|-14--_
    (GPIO_GEN3) GPIO22--15-|O O|-16-----GPIO23 (GPIO_GEN4)
                  +3V3--17-|O O|-18-----GPIO24 (GPIO_GEN5)
     (SPI_MOSI) GPIO10--19-|O.O|-20--_
     (SPI_MISO) GPIO9 --21-|O O|-22-----GPIO25 (GPIO_GEN6)
     (SPI_SCLK) GPIO11--23-|O O|-24-----GPIO8  (SPI_C0_N)
                       -25-|O O|-26-----GPIO7  (SPI_C1_N)
       (EEPROM) ID_SD---27-|O O|-28-----ID_SC Reserved for ID EEPROM
                GPIO5---29-|O.O|-30--_
                GPIO6---31-|O O|-32-----GPIO12
                GPIO13--33-|O O|-34--_
                GPIO19--35-|O O|-36-----GPIO16
                GPIO26--37-|O O|-38-----GPIO20
                       -39-|O O|-40-----GPIO21
                           '---'
                       40W 0.1" PIN HDR






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
