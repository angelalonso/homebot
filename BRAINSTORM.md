# TL;DR
- just run make
  - Some fuzzy intelligence behind the makefile determines whether it will run in a simulation, or the actual bot
    - If there is Internet, it will rebuild in any case, if not, it will take the latest build
      - Some autocleanup for the builds wouold be nice too
    - We probably need a config file to identify the machine it will run in, the other ones will just simulate

# PHASES
## 1: The robot code can run on simulation and on the robot'S hardware itself
## 2: The robot can be left in a room and it maps its surroundings automatically
## 3: The robot can receive a point in that map and it will navigate the room to get there


