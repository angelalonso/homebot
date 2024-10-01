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
  if [[ "$1" == "" ]]; then
    MODE="simulate"
  else
    MODE="$1"
  fi
  
  # If we have access to Internet, it will rebuild in any case, if not, it will take the latest build
  test_online
}



check_mode $1

cargo build --features $MODE

if [[ ${MODE} == "live" ]]; then
  echo "RUNNING LIVE"
elif [[ ${MODE} == "simulate" ]]; then
	mkdir -p simulation/controllers/rust_controller/
	cp target/debug/homebot simulation/controllers/rust_controller/rust_controller
	cp cfg.yaml simulation/controllers/rust_controller/
	webots simulation/worlds/homebot_simulation_world.wbt
elif [[ ${MODE} == "clisimulate" ]]; then
  cargo run --features $MODE
else
  echo ${MODE} NOT RECOGNIZED
fi


