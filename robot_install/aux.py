from cryptography.hazmat.primitives import serialization as crypto_serialization
from cryptography.hazmat.primitives.asymmetric import rsa
from cryptography.hazmat.backends import default_backend as crypto_default_backend

from shutil import which
import logging
import yaml
from pathlib import Path
import os.path


def is_installed(name):
    """Check whether `name` is on PATH and marked as executable."""
    return which(name) is not None


class CustomFormatter(logging.Formatter):
    grey = "\x1b[38;20m"
    blue = "\x1b[34;20m"
    yellow = "\x1b[33;20m"
    red = "\x1b[31;20m"
    bold_red = "\x1b[31;1m"
    reset = "\x1b[0m"
    # format = "%(asctime)s - %(name)s - %(levelname)s - %(message)s (%(filename)s:%(lineno)d)"
    format = "%(levelname).1s|%(asctime)s %(message)s"

    FORMATS = {
        logging.DEBUG: blue + format + reset,
        logging.INFO: grey + format + reset,
        logging.WARNING: yellow + format + reset,
        logging.ERROR: red + format + reset,
        logging.CRITICAL: bold_red + format + reset,
    }

    def format(self, record):
        log_fmt = self.FORMATS.get(record.levelno)
        formatter = logging.Formatter(log_fmt)
        return formatter.format(record)


class bcolors:
    LILA = '\033[95m'
    BLUE = '\033[94m'
    CYAN = '\033[96m'
    GREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'


def printfmt(msgtype, msg):
    if msgtype == "lila":
        msgt = f"{bcolors.LILA}"
    elif msgtype == "blue":
        msgt = f"{bcolors.BLUE}"
    elif msgtype == "cyan":
        msgt = f"{bcolors.CYAN}"
    elif msgtype == "green":
        msgt = f"{bcolors.GREEN}"
    elif msgtype == "yellow":
        msgt = f"{bcolors.WARNING}"
    elif msgtype == "red":
        msgt = f"{bcolors.FAIL}"
    elif msgtype == "bold":
        msgt = f"{bcolors.BOLD}"
    elif msgtype == "underline":
        msgt = f"{bcolors.UNDERLINE}"
    else:
        msgt = f"{bcolors.ENDC}"
    print(msgt + msg + f"{bcolors.ENDC}")


def read_cfg(filename):
    result = yaml.safe_load(Path(filename).read_text())
    return result


def write_cfg(filename, data):
    new_data = data.copy()
    new_data.pop('privkey', None)
    new_data.pop('pubkey', None)
    with open(filename, 'w') as yaml_file:
        yaml.dump(new_data, yaml_file, default_flow_style=False)

def read_bytesfile(file):
    with open(file, "rb") as f:
        byte = f.read(1)
        while byte:
            byte = f.read(1)

def get_sshkeypair(logger, cfg):
    pubkey = 'homebot.pub'
    privkey = 'homebot.priv'

    if os.path.isfile(pubkey) and os.path.isfile(privkey):
        fpb = open(pubkey, "r")
        cfg['pubkey'] = fpb.read()
        fpv = open(privkey, "r")
        cfg['privkey'] = fpv.read()

    else:
        key = rsa.generate_private_key(
            backend=crypto_default_backend(),
            public_exponent=65537,
            key_size=2048
        )

        cfg['privkey'] = key.private_bytes(
            crypto_serialization.Encoding.PEM,
            crypto_serialization.PrivateFormat.PKCS8,
            crypto_serialization.NoEncryption()
        )

        cfg['pubkey'] = key.public_key().public_bytes(
            crypto_serialization.Encoding.OpenSSH,
            crypto_serialization.PublicFormat.OpenSSH
        )
        pubfile = open(pubkey, 'wb')
        privfile = open(privkey, 'wb')
        pubfile.write(cfg['pubkey'])
        privfile.write(cfg['privkey'])
        os.chmod(privkey, 0o600)
        pubfile.close()
        privfile.close()
    return cfg

    # TODO: load them for use with ssh (copy public over for the user)

