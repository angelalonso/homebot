import logging
import sys

import aux
from aux import printfmt as pfmt


def main():
    # "Start" the logging facility
    logger = logging.getLogger(__name__)
    ch = logging.StreamHandler()
    ch.setFormatter(aux.CustomFormatter())
    logger.setLevel("DEBUG")
    logger.addHandler(ch)

    cfg_file = "cfg.yml"
    cfg = aux.read_cfg(cfg_file)

    # Guide user through preparations
    # We avoid repeating steps that were done
    if 'steps_done' not in cfg: 
        cfg['steps_done'] = 0
    if cfg['steps_done'] < 1:
        pfmt("lila", "REQUIREMENTS:")
        pfmt("normal", " - A laptop running linux (ubuntu maybe?)")
        pfmt("normal", " - MicroSD Card, Minimum 16G")
        pfmt("normal", " - Raspberry Pi 3B+ or newer")
        pfmt("normal", " - A home Router")
        pfmt("normal", " - A Network cable, RJ45")
        pfmt("normal", " - Debian image for armhf")
        pfmt("yellow", "   - https://raspi.debian.net/tested/20231109_raspi_3_bookworm.img.xz")
        pfmt("red", "Press <ENTER> to continue when you have them all, CTRL+C to exit")
        input()
        cfg['steps_done'] = 1
    else:
        pfmt("lila", "- REQUIREMENTS already met")

    if cfg['steps_done'] < 2:
        pfmt("lila", "PREPARATION STEPS: Burn the base image")
        pfmt("normal", " - Unzip the Debian image")
        pfmt("yellow", "   - unxz 20231109_raspi_3_bookworm.img.xz")
        pfmt("normal", " - Connect the MicroSD to your computer")
        pfmt("normal", " - Get the MAIN device ID for the MicroSD. E.g.:mmcblk0 and not mmcblk0p1")
        pfmt("yellow", "   - lsblk")
        pfmt("normal", " - Copy over the image to the MicroSD")
        pfmt("yellow", "  - sudo dd if=20231109_raspi_3_bookworm.img of=</dev/whatever> status=progress")
        pfmt("normal", " - Once finished, mount the two partitions to /media/<USER>/RASPIFIRM/ and /media/<USER>/RASPIROOT ?")
        pfmt("red", "Press <ENTER> to continue when you are ready and the partitions are mounted, CTRL+C to exit")
        input()
        cfg['steps_done'] = 2
    else:
        pfmt("lila", "- Base Image was already burnt")

    if cfg['steps_done'] < 3:
        pfmt("lila", "PREPARATION STEPS: Modify the Debian defaults")
        pfmt("normal", " - Enable SSH")
        pfmt("yellow", "  - touch /media/<USER>/RASPIFIRM/ssh")
        pfmt("normal", " - Install dependencies to run wemu and chroot")
        pfmt("yellow", "  - sudo apt install qemu qemu-user-static binfmt-support")
        pfmt("normal", " - Make sure our script is executable")
        pfmt("yellow", "  - chmod +x chroot_pi.sh")
        pfmt("normal", " - Run the chroot script (Thanks to https://gist.github.com/htruong/7df502fb60268eeee5bca21ef3e436eb)")
        pfmt("yellow", "  - ./chroot_pi.sh /dev/mmcblk0")
        pfmt("normal", " - Make sure you are no longer on your machine, then edit the sshd config")
        pfmt("yellow", "  - vi /etc/ssh/sshd_config")
        pfmt("blue", "  PermitRootLogin yes")
        pfmt("normal", " - Save and get out of the chroot")
        pfmt("yellow", "  - exit ")
        pfmt("red", "Press <ENTER> to continue when you are have exited chroot, CTRL+C to exit")
        input()
        cfg['steps_done'] = 3
    else:
        pfmt("lila", "- Debian defaults were changed")

    if cfg['steps_done'] < 4:
        pfmt("lila", "PREPARATION STEPS: Plug the Raspberry and boot it")
        pfmt("normal", " - Now unmount the /media partitions,")
        pfmt("normal", "     connect the MicroSD to the Raspberry,")
        pfmt("normal", "     connect the Raspberry to your Router with the RJ45 cable,")
        pfmt("normal", "     plug the Raspberry in, have it boot")
        pfmt("red", "Press <ENTER> to continue when your Raspberry is connected and started, CTRL+C to exit")
        input()
        cfg['steps_done'] = 4
    else:
        pfmt("lila", "- Raspberry already booted")

    if cfg['steps_done'] < 5:
        pfmt("lila", "PREPARATION STEPS: Add your Raspberry Pi IP")
        pfmt("normal", " - Find the IP of your Raspberry once it has connected")
        pfmt("normal", "  - Modify the following to the IP range your router uses:")
        pfmt("yellow", "  - nmap -sP 192.168.1.0/24")
        pfmt("red", "Press <ENTER> when you have found the IP")
        input()
        eth_ip = input("Enter your IP: ")
        cfg['eth_ip'] = eth_ip
        cfg['steps_done'] = 5
    else:
        pfmt("lila", "- Raspberrys IP is known: " + cfg['eth_ip'])

    if cfg['steps_done'] < 6:
        pfmt("lila", "PREPARATION STEPS: Create a user")
        user = input("Enter the name of the user you want on the Raspberry: ")
        passwd = input("Enter a password for that user: ")
        cfg['user'] = user
        cfg['pass'] = passwd
        cfg['steps_done'] = 6
    else:
        pfmt("lila", "- User (and password) is known: " + cfg['user'])

    aux.write_cfg(cfg_file, cfg)
    # Give us a TEMPORARY password to use



    logger.info("ALL DONE.")


if __name__ == "__main__":
    main()
