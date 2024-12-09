from aux import printfmt as pfmt
from getpass import getpass
import ssh
import aux

def step_1(cfg):
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
    return cfg


def step_2(cfg):
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
    return cfg


def step_3(cfg):
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
        pfmt("normal", " - Then change root's password to your liking")
        pfmt("yellow", "  - passwd")
        pfmt("normal", " - Now we will need to store that pass for the next phase (written into cfg.yml)")
        rootpasswd = getpass("Please write here your root password for that Raspberry: ")
        cfg['rootpasswd'] = rootpasswd
        pfmt("normal", " - Save and get out of the chroot")
        pfmt("yellow", "  - exit ")
        pfmt("red", "Press <ENTER> to continue when you are have exited chroot, CTRL+C to exit")
        input()
        cfg['steps_done'] = 3
    else:
        pfmt("lila", "- Debian defaults were changed")
    return cfg


def step_4(cfg):
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
    return cfg


def step_5(cfg, cfg_file):
    if cfg['steps_done'] < 5:
        pfmt("lila", "PREPARATION STEPS: Add your Raspberry Pi IP")
        pfmt("normal", " - Find the IP of your Raspberry once it has connected")
        pfmt("normal", "  - You can modify the following to the IP range your router uses and run it:")
        pfmt("yellow", "  - nmap -sP 192.168.1.0/24")
        pfmt("red", "Now go and find the IP, ")
        eth_ip = input(" then enter your IP here: ")
        pfmt("red", "Using 22 as SSH Port, if you modified it on the Debian defaults, please edit " + cfg_file)
        cfg['eth_ip'] = eth_ip
        cfg['ssh_port'] = 22
        cfg['steps_done'] = 5
    else:
        pfmt("lila", "- Raspberrys IP is known: " + cfg['eth_ip'])
    return cfg


def step_6(logger, cfg):
    if cfg['steps_done'] < 6:
        pfmt("lila", "PREPARATION STEPS: Create a user")
        user = input("Enter the name of the user you want on the Raspberry: ")
        cfg['user'] = user
        ssh.create_user(logger,
                        cfg['eth_ip'],
                        cfg['ssh_port'],
                        'root',
                        cfg['rootpasswd'],
                        cfg['user'],
                        cfg['pubkey']
                        )
        cfg['steps_done'] = 6
    else:
        pfmt("lila", "- User (and password) is known: " + cfg['user'])
    return cfg


def step_7(logger, cfg):
    if cfg['steps_done'] < 7:
        pfmt("lila", "PREPARATION STEPS: Secure access")
        ssh.secure_access(logger,
                          cfg['eth_ip'],
                          cfg['ssh_port'],
                          cfg['user'],
                          cfg['privkeyfile']
                          )
        cfg['rootpasswd'] = ""
        cfg['passwd'] = ""
        cfg['pass'] = ""
        cfg['steps_done'] = 7
    else:
        pfmt("lila", "- Access is secured. No root SSH access and no passwords on your config")
    return cfg


# TODO: this:
def step_8(logger, cfg):
    if cfg['steps_done'] < 8:
        pfmt("lila", "PREPARATION STEPS: change connection to wifi")
        #ssh.config_wifi(logger,
        #                 cfg['eth_ip'],
        #                 cfg['ssh_port'],
        #                 cfg['user'],
        #                 cfg['privkeyfile']
        #                 )
        #cfg['steps_done'] = 8
    else:
        pfmt("lila", "- The Robot is now accessible through wifi")
    return cfg


def step_9(logger, cfg):
    if cfg['steps_done'] < 9:
        pfmt("lila", "PREPARATION STEPS: install packages")
        ssh.install_pkgs(logger,
                         cfg['eth_ip'],
                         cfg['ssh_port'],
                         cfg['user'],
                         cfg['privkeyfile']
                         )
        cfg['steps_done'] = 9
    else:
        pfmt("lila", "- Required packages have been installed")
    return cfg


def step_10(logger, cfg):
    if cfg['steps_done'] < 10:
        pfmt("lila", "PREPARATION STEPS: copy Homebot's code to the Robot")
        ssh.git_clone(logger,
                      cfg['eth_ip'],
                      cfg['ssh_port'],
                      cfg['user'],
                      cfg['privkeyfile']
                      )
        cfg['steps_done'] = 10
    else:
        pfmt("lila", "- Homebot's code has been copied to the Robot")
    return cfg


# TODO: this:
def step_11(logger, cfg):
    if cfg['steps_done'] < 11:
        pfmt("lila", "PREPARATION STEPS: Create a system service to run the Robot")
        ssh.homebot_service(logger,
                            cfg['eth_ip'],
                            cfg['ssh_port'],
                            cfg['user'],
                            cfg['privkeyfile']
                            )
        #cfg['steps_done'] = 9000 # 900 because it's the last one
    else:
        pfmt("lila", "- There is a System Service ready to run Homebot at the Robot")
    return cfg


# TODO: this:
def step_refresh(logger, cfg):
    if cfg['steps_done'] == 9000:
        pfmt("lila", "PREPARATION STEPS: Create a system service to run the Robot")
        ssh.homebot_refresh(logger,
                            cfg['eth_ip'],
                            cfg['ssh_port'],
                            cfg['user'],
                            cfg['privkeyfile']
                            )
    else:
        pfmt("lila", "- Homebot refresh done")
    return cfg

def step_test(logger, cfg):
    ssh.test(logger,
             cfg['eth_ip'],
             cfg['ssh_port'],
             cfg['user'],
             cfg['privkeyfile']
             )
    return cfg
