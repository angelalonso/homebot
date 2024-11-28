from aux import printfmt as pfmt

def step_1():
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

def step_2():
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

def step_3():
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

def step_4():
    pfmt("lila", "PREPARATION STEPS: Plug the Raspberry and boot it")
    pfmt("normal", " - Now unmount the /media partitions,")
    pfmt("normal", "     connect the MicroSD to the Raspberry,")
    pfmt("normal", "     connect the Raspberry to your Router with the RJ45 cable,")
    pfmt("normal", "     plug the Raspberry in, have it boot")
    pfmt("red", "Press <ENTER> to continue when your Raspberry is connected and started, CTRL+C to exit")
    input()

def step_5():
    pfmt("lila", "PREPARATION STEPS: Add your Raspberry Pi IP")
    pfmt("normal", " - Find the IP of your Raspberry once it has connected")
    pfmt("normal", "  - Modify the following to the IP range your router uses:")
    pfmt("yellow", "  - nmap -sP 192.168.1.0/24")
    pfmt("red", "Press <ENTER> when you have found the IP")
    input()

def step_6():
    pfmt("lila", "PREPARATION STEPS: Create a user")
