#!/usr/bin/env bash


#- just run make
#  - Some fuzzy intelligence behind the makefile determines whether it will run in a simulation, or the actual bot
#      - Some autocleanup for the builds wouold be nice too
#    - We probably need a config file to identify the machine it will run in, the other ones will just simulate

test_online() {
  ping google.com -c1 > /dev/null 2>&1
  if [[ $? -eq 0 ]]; then
    am_online=True
  else
    am_online=False
  fi
}

check_mode() {
  # If we are on the robot, we run live
  THIS_MACHINE=$(hostname)
  if [[ ${ROBOT_MACHINE_NAME} == ${THIS_MACHINE} ]]; then
    MODE="live"
  else
    MODE="simulate"
  fi
  
  # If we have access to Internet, it will rebuild in any case, if not, it will take the latest build
  test_online
  if [[ $am_online == True ]]; then
    NEEDS_BUILD=True
  else
    NEEDS_BUILD=False
  fi
}



source ./variables


check_mode
if [[ $NEEDS_BUILD == True ]];then
	cargo build --features $MODE
fi

if [[ ${MODE} == "live" ]]; then
  echo "RUNNING LIVE"
elif [[ ${MODE} == "simulate" ]]; then
  echo "TESTING LOCALLY"
	mkdir -p simulation/controllers/rust_controller/
	cp target/debug/homebot simulation/controllers/rust_controller/rust_controller
	webots simulation/worlds/homebot_simulation_world.wbt
else
  echo ${MODE} NOT RECOGNIZED
fi


