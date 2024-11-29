import paramiko

def create_user(logger, raspi_ip, raspi_port, sshuser, sshpasswd, newuser, newpasswd):
    client = paramiko.SSHClient()
    client.load_system_host_keys()
    client.connect(hostname=raspi_ip, port=raspi_port, username=sshuser, password=sshpasswd, allow_agent=False)
    cmd_1 = 'adduser ' + newuser + ' --disabled-password --gecos "" || true; echo ' + newuser + ':' + newpasswd + ' | chpasswd'
    ssh_stdin, ssh_stdout, ssh_stderr = client.exec_command(cmd_1)
    stdout = ssh_stdout.readlines()
    errcode = ssh_stdout.channel.recv_exit_status() 
    stderr = ssh_stderr.readlines()
    if errcode != 0:
        logger.error("ERROR executing: " + cmnd)
        for e in stderr:
            logger.error(e)
        for o in stdout:
            logger.info(o)
    else: 
        logger.info("User " + newuser + " successfully created")
        for o in stdout:
            logger.info(o)
