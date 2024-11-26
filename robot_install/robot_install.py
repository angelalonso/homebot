import logging
import sys

import aux


def main():
    # "Start" the logging facility
    logger = logging.getLogger(__name__)
    ch = logging.StreamHandler()
    ch.setFormatter(aux.CustomFormatter())
    logger.setLevel("DEBUG")
    logger.addHandler(ch)

    # Check all requirements, then only continue if all are satisfied
    set_to_go = True
    # balena-etcher - TODO: don't stick to balena-etcher, 
    #   rather "have image downloaded from here, burn it, 
    #   maybe with balena-etcher, then mount it, then press enter"
    if not aux.is_installed("balena-etcher"):
        set_to_go = False
        logger.error("ERROR: etcher not found on this machine!\
                \n  You may want to\
                \n  - download it from https://etcher.balena.io/#download-etcher\
                \n  - unzip to a folder of your liking\
                \n  - make a link into $HOME/.local/bin:\
                \n ln -s $HOME/Downloads/balenaEtcher-linux-x64-1.19.25/balenaEtcher-linux-x64/balena-etcher $HOME/.local/bin/balena-etcher")
        sys.exit(2)

    logger.info("Requirements:\n\
            - Now run balena-etcher")
    logger.info("ALL DONE.")
    # Do you have a microSD with Debian ARMHF installed?
    # Do you want to do it now?
#    sudo dd if=20231109_raspi_3_bookworm.img of=/dev/mmcblk0 status=progress
# Mounted at /media/aaf/RASPIFIRM/ and /media/aaf/RASPIROOT ? 
# touch /media/aaf/RASPIFIRM/ssh
# Do you have a key pair? do you want to create it?
# sudo vim /media/aaf/RASPIFIRM/sysconf.txt
# root_authorized_key=ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINmEzgdeRcX5iGbVmeYT7X0fyDzAh58juL6xYAwyPHCR alonsofonseca.angel@gmail.com
# save, connect to raspberry
# Connect raspi with RJ45, find it
# nmap -sP 192.168.1.0/24 - ask if needs to be used, then get list before starting, then compare and get the new IP, asking for confirmation step by step
# ssh pi@<IP> - raspberry doesnt work


if __name__ == "__main__":
    main()
