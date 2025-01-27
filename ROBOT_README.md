# Robot Base software

We run a Debian on the Raspberry pi. This part explains what is done there.
12.2 arm64

# Preparation
- new user and password, add pubkey
- user, pass and key on cfg.yml
- sudoers -> no password needed
- port to 21012
- update, upgrade
- install vim git curl build-essential libclang-dev
- curl https://sh.rustup.rs -sSf | sh
 - default
- source $HOME/.cargo/env


## Automatic Installation
-- TO BE TESTED --

- Get to ./robot_install
- Run:
  - pipenv install
  - pipenv run python3 robot_install.py
- ...and follow instructions 

# TO BE DONE
- Install Rust
- Download this code to the machine
- Connect to wifi and test
- Create a Service to run the robot
- Once all steps have been ran, use the last one to update code and run
