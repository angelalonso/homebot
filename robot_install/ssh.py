import paramiko
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
    client.connect(hostname=ip,
                   port=port,
                   username=user,
                   password=passwd,
                   allow_agent=False
                   )
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
        logger.error("ERROR executing: " + cmd_1)
        for e in stderr:
            logger.error(e)
        for o in stdout:
            logger.info(o)
    else: 
        logger.info("User " + newuser + " successfully created")
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
