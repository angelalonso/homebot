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
# Connect raspi with RJ45, find it
# ssh pi@<IP> - raspberry


if __name__ == "__main__":
    main()
