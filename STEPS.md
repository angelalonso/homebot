# Derived Requirements
[ OK  ] - There is a Robot
[Check]  - It has a computer, the Raspberry pi (Raspi)
[TBDoc]    - The Raspi has an operating system - see 00_preparations/robot_install
[TBDoc]    - The Raspi is connected to the LAN, preferably through wifi
[TBDoc]    - The Raspi has a battery
[ OK  ]- The Robot moves
[TBDoc]  - It has wheels
[TBDoc]  - It has motors to move those wheels
[TBDoc]  - It has a Motor controller (L298N)
[TBDoc]  - It has batteries for that Motor controller
[TBDoc]  - The Raspi manages the L298N
[ OK  ]    - The Raspi can turn the wheels on and off - 03_motors_rs
[ OK  ]      - (Non-derived req) The Raspi can control GPIO from a Python script - 01_pythontest
[ OK  ]      - (Non-derived req) The Raspo can control GPIO from a Rust script - 01_rusttest
[ OK  ]      - The Raspi has a program (homebot) that can make the L298N move the Motors
[ OK  ]        - (Non-derived req) homebot is written in Rust - 10_integration
[ OK  ]        - (Non-derived req) there is a way to install homebot - homebotctl
[ OK  ]          - homebot can be installed from my laptop - homebotctl
[ OK  ]            - my laptop can SSH into the Raspi - homebotctl
[ OK  ]            - my laptop can cross-compile for the Raspi - homebotctl
[ OK  ]          - homebot can be tested locally on my laptop - homebotctl
[ OK  ]          - homebot can be simulated locally on my laptop - homebotctl
[TBDoc]            - the simulation is done via webots
[TBDoc]  - It has an Arduino that gets data from sensors and sends it to the Raspbi
[TBDoc]    - We can work with arduino code
[ OK  ]      - Install Arduino IDE on your laptop - 00_preparations/00A_Arduino/README.md
[TBDoc]    - The Arduino is connected to a distance sensor and can get data from it
[ OK  ]    - The Arduino can send that data every x milliseconds - 00_preparations/00AD_Ard_Faster
[ OK  ]    - The Raspi can read that data and print it on Python - 02_serial_py
[ OK  ]    - The Raspi can read that data and print it on Rust - 02_serial_rs
[  -  ]- The robot is independent on how and when to move
[  -  ]  - TBD
