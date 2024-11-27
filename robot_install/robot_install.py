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

    aux.printfmt("REQUIREMENTS:", aux.bcolors.HEADER)
    logger.info("Requirements:\n\
            - Now run balena-etcher")
# - MicroSD Card, I have used 16G but maybe 8G is enough.
# - Raspberry pi 3B+
# 1. Download https://raspi.debian.net/tested/20231109_raspi_3_bookworm.img.xz from https://raspi.debian.net/tested-images/ 
# 1. unxz 20231109_raspi_3_bookworm.img.xz
    # Do you have a microSD with Debian ARMHF installed?
    # Do you want to do it now?
#    sudo dd if=20231109_raspi_3_bookworm.img of=/dev/mmcblk0 status=progress
# Mounted at /media/aaf/RASPIFIRM/ and /media/aaf/RASPIROOT ? 
# touch /media/aaf/RASPIFIRM/ssh
# Do you have a key pair? do you want to create it?
# ## OPTION 1 -DID NOT WORK
# sudo vim /media/aaf/RASPIFIRM/sysconf.txt
# root_authorized_key=ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINmEzgdeRcX5iGbVmeYT7X0fyDzAh58juL6xYAwyPHCR alonsofonseca.angel@gmail.com
# ## OPTION 2 -CHROOT
# sudo apt install qemu qemu-user-static binfmt-support
# chmod +x chroot_pi.sh
#     Thanks to https://gist.github.com/htruong/7df502fb60268eeee5bca21ef3e436eb
# ./chroot_pi.sh /dev/mmcblk0
# vi /etc/ssh/sshd_config
#   PermitRootLogin yes

# ## Common
# save, connect to raspberry
# Connect raspi with RJ45, find it
# nmap -sP 192.168.1.0/24 - ask if needs to be used, 
#   then get list before starting, then compare and get the new IP, asking for confirmation step by step
# ssh pi@<IP> - raspberry doesnt work
    if set_to_go:
        logger.info("Applying...")

    logger.info("ALL DONE.")


if __name__ == "__main__":
    main()
