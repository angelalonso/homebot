#!/usr/bin/env bash


set -o errexit
set -o nounset
set -o pipefail
set -o xtrace


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
    MODE="live"
  else
    MODE="$1"
  fi
  
  # If we have access to Internet, it will rebuild in any case, if not, it will take the latest build
  #test_online
}



check_mode $1


if [[ ${MODE} == "build" ]]; then
  echo "BUILDING FOR RASPBERRY PI"
  cargo build --features live --release --target=armv7-unknown-linux-gnueabihf
elif [[ ${MODE} == "live" ]]; then
  cargo build --features $MODE # to be removed and use the built program instead
  echo "RUNNING LIVE"
elif [[ ${MODE} == "sim" ]]; then
  cargo build --features $MODE --release
	mkdir -p simulation/controllers/rust_controller/
	cp target/debug/homebot simulation/controllers/rust_controller/rust_controller
	cp cfg.yaml simulation/controllers/rust_controller/
	webots simulation/worlds/homebot_simulation_world.wbt
elif [[ ${MODE} == "test" ]]; then
  cargo test --features test -- --nocapture
else
  echo ${MODE} NOT RECOGNIZED
fi


