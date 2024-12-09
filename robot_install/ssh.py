import paramiko
import paramiko.ssh_exception
import os


def run(ip, port, user, privkeyfile, cmd):
    client = paramiko.SSHClient()
    client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
    client.connect(hostname=ip,
                   port=port,
                   username=user,
                   key_filename=os.getcwd() + "/" + privkeyfile,
                   look_for_keys=False
                   )
    ssh_stdin, ssh_stdout, ssh_stderr = client.exec_command(cmd)
    stdout = ssh_stdout.readlines()
    errcode = ssh_stdout.channel.recv_exit_status()
    stderr = ssh_stderr.readlines()
    return (stdout, errcode, stderr)


def run_w_pass(ip, port, user, passwd, cmd):
    client = paramiko.SSHClient()
    client.load_system_host_keys()
    try:
        client.connect(hostname=ip,
                       port=port,
                       username=user,
                       password=passwd,
                       allow_agent=False
                       )
    except paramiko.ssh_exception.SSHException as e:
        return ("", "SSH Exception", e)
    ssh_stdin, ssh_stdout, ssh_stderr = client.exec_command(cmd)
    stdout = ssh_stdout.readlines()
    errcode = ssh_stdout.channel.recv_exit_status()
    stderr = ssh_stderr.readlines()
    return (stdout, errcode, stderr)


def create_user(logger,
                raspi_ip,
                raspi_port,
                sshuser,
                sshpasswd,
                newuser,
                newpubkey
                ):
    homessh = '/home/' + newuser + '/.ssh'
    cmd_1 = 'adduser ' + newuser + ' --disabled-password --gecos "" || true; mkdir -p ' + homessh + ';echo ' + newpubkey.decode("utf-8") + ' >> ' + homessh + '/authorized_keys; usermod -aG sudo ' + newuser + '; apt update && apt install -y sudo; echo "' + newuser + ' ALL=(ALL:ALL) NOPASSWD: ALL" | sudo tee /etc/sudoers.d/' + newuser

    (stdout, errcode, stderr) = run_w_pass(raspi_ip, raspi_port, sshuser, sshpasswd, cmd_1)

    if errcode != 0:
        if errcode == "SSH Exception":
            logger.error("Host " + raspi_ip + " is probably not on known_hosts! Try connecting manually with ssh root@" + raspi_ip + " -p " + str(raspi_port) + ", and accept to continue connecting.")

        else:
            logger.error("ERROR executing: " + cmd_1)
            for e in stderr:
                logger.error(e)
            for o in stdout:
                logger.info(o)
    else:
        logger.info("User " + newuser + " successfully created")
        for o in stdout:
            logger.info(o)


def secure_access(logger,
                  raspi_ip,
                  raspi_port,
                  user,
                  keyfile,
                  ):
    cmd_1 = 'sudo sed -i "s/.*PasswordAuthentication.*/PasswordAuthentication no/g" /etc/ssh/sshd_config; sudo sed -i "s/.*PermitRootLogin.*/PermitRootLogin no/g" /etc/ssh/sshd_config; sudo systemctl restart sshd'

    (stdout, errcode, stderr) = run(raspi_ip, raspi_port, user, keyfile, cmd_1)

    if errcode != 0:
        if errcode == "SSH Exception":
            logger.error("Host " + raspi_ip + " is probably not on known_hosts! Try connecting manually with ssh root@" + raspi_ip + " -p " + str(raspi_port) + ", and accept to continue connecting.")

        else:
            logger.error("ERROR executing: " + cmd_1)
            for e in stderr:
                logger.error(e)
            for o in stdout:
                logger.info(o)
    else:
        logger.info("SSH access allowed only to regular users with key")
        for o in stdout:
            logger.info(o)


# TODO: this:
def config_wifi(logger,
                raspi_ip,
                raspi_port,
                user,
                keyfile,
                ):
    # Config wifi user+pass
    # connect and test it worked
    # get new IP and save it to conf
    cmd_1 = ''

    (stdout, errcode, stderr) = run(raspi_ip, raspi_port, user, keyfile, cmd_1)

    if errcode != 0:
        if errcode == "SSH Exception":
            logger.error("Host " + raspi_ip + " is probably not on known_hosts! Try connecting manually with ssh root@" + raspi_ip + " -p " + str(raspi_port) + ", and accept to continue connecting.")

        else:
            logger.error("ERROR executing: " + cmd_1)
            for e in stderr:
                logger.error(e)
            for o in stdout:
                logger.info(o)
    else:
        logger.info("The Robot is connected to a WLAN")
        for o in stdout:
            logger.info(o)


def install_pkgs(logger,
                 raspi_ip,
                 raspi_port,
                 user,
                 keyfile,
                 ):
    pkgs = 'vim \
            git'
    cmd_1 = 'sudo apt update && sudo apt upgrade -y && sudo apt install -y ' + pkgs

    (stdout, errcode, stderr) = run(raspi_ip, raspi_port, user, keyfile, cmd_1)

    if errcode != 0:
        if errcode == "SSH Exception":
            logger.error("Host " + raspi_ip + " is probably not on known_hosts! Try connecting manually with ssh root@" + raspi_ip + " -p " + str(raspi_port) + ", and accept to continue connecting.")

        else:
            logger.error("ERROR executing: " + cmd_1)
            for e in stderr:
                logger.error(e)
            for o in stdout:
                logger.info(o)
    else:
        logger.info("Required packages have been installed")
        for o in stdout:
            logger.info(o)


def git_clone(logger,
              raspi_ip,
              raspi_port,
              user,
              keyfile,
              ):
    cmd_1 = 'cd $HOME && if [ ! -d "homebot"  ] ; then git clone https://github.com/angelalonso/homebot fi'

    (stdout, errcode, stderr) = run(raspi_ip, raspi_port, user, keyfile, cmd_1)

    if errcode != 0:
        if errcode == "SSH Exception":
            logger.error("Host " + raspi_ip + " is probably not on known_hosts! Try connecting manually with ssh root@" + raspi_ip + " -p " + str(raspi_port) + ", and accept to continue connecting.")

        else:
            logger.error("ERROR executing: " + cmd_1)
            for e in stderr:
                logger.error(e)
            for o in stdout:
                logger.info(o)
    else:
        logger.info("Homebot code has been installed to the Robot")
        for o in stdout:
            logger.info(o)


# TODO: this:
def homebot_service(logger,
                    raspi_ip,
                    raspi_port,
                    user,
                    keyfile,
                    ):
    cmd_1 = ''

    (stdout, errcode, stderr) = run(raspi_ip, raspi_port, user, keyfile, cmd_1)

    if errcode != 0:
        if errcode == "SSH Exception":
            logger.error("Host " + raspi_ip + " is probably not on known_hosts! Try connecting manually with ssh root@" + raspi_ip + " -p " + str(raspi_port) + ", and accept to continue connecting.")

        else:
            logger.error("ERROR executing: " + cmd_1)
            for e in stderr:
                logger.error(e)
            for o in stdout:
                logger.info(o)
    else:
        logger.info("The Homebot service has been created")
        for o in stdout:
            logger.info(o)


# TODO: this:
def homebot_refresh(logger,
                    raspi_ip,
                    raspi_port,
                    user,
                    keyfile,
                    ):
    cmd_1 = ''

    (stdout, errcode, stderr) = run(raspi_ip, raspi_port, user, keyfile, cmd_1)

    if errcode != 0:
        if errcode == "SSH Exception":
            logger.error("Host " + raspi_ip + " is probably not on known_hosts! Try connecting manually with ssh root@" + raspi_ip + " -p " + str(raspi_port) + ", and accept to continue connecting.")

        else:
            logger.error("ERROR executing: " + cmd_1)
            for e in stderr:
                logger.error(e)
            for o in stdout:
                logger.info(o)
    else:
        logger.info("Homebot code and running program have been refreshed")
        for o in stdout:
            logger.info(o)


def test(logger,
         raspi_ip,
         raspi_port,
         user,
         keyfile,
         ):
    cmd_1 = 'sudo whoami'
    (stdout, errcode, stderr) = run(raspi_ip, raspi_port, user, keyfile, cmd_1)

    if errcode != 0:
        logger.error("ERROR executing: " + cmd_1)
        for e in stderr:
            logger.error(e)
        for o in stdout:
            logger.info(o)
        logger.error("Remember to ssh-add " + keyfile)
    else:
        for o in stdout:
            logger.info(o)
