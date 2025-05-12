# Installing Arduino at your local laptop

Download the zip file from https://www.arduino.cc/en/software/
unzip arduino-ide_2.3.6_Linux_64bit.zip
sudo chown root:root ./arduino-ide_2.3.6_Linux_64bit/chrome-sandbox
sudo chmod 4755 ./arduino-ide_2.3.6_Linux_64bit/chrome-sandbox
sudo adduser <YOURUSER> dialout
run it with ./arduino-ide_2.3.6_Linux_64bit/arduino-ide

## Using the Arduino

You will have to connect it to the laptop to load and test programs.

You will then have to connect it to the Raspi to actually have the robot use it.

# Cross compiling at your Laptop 
Provided you run Ubuntu:
- sudo apt update
- sudo apt install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
- Create or edit ~/.cargo/config.toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
- rustup target add aarch64-unknown-linux-gnu
- Then you can cross compile with:
  - cargo build --release --target=aarch64-unknown-linux-gnu
  - , or just use homebotctl:
    - cd homebotctl 
    - adapt code_path at ctlcfg.yml
    - cargo run build
