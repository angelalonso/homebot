import paramiko


def run(ip, port, user, passwd, cmd):
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
    cmd_1 = 'adduser ' + newuser + ' --disabled-password --gecos "" || true; mkdir -p ' + homessh + ';echo ' + newpubkey.decode("utf-8") + ' >> ' + homessh + '/authorized_keys'

    (stdout, errcode, stderr) = run(raspi_ip, raspi_port, sshuser, sshpasswd, cmd_1)

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
         sshuser,
         sshpasswd,
         newuser,
         newpubkey
         ):
    cmd_1 = 'whoami'
    (stdout, errcode, stderr) = run(raspi_ip, raspi_port, sshuser, sshpasswd, cmd_1)

    if errcode != 0:
        logger.error("ERROR executing: " + cmd_1)
        for e in stderr:
            logger.error(e)
        for o in stdout:
            logger.info(o)
    else: 
        for o in stdout:
            logger.info(o)
