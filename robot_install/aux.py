from shutil import which
import logging
import yaml
from pathlib import Path


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
    with open(filename, 'w') as yaml_file:
        yaml.dump(data, yaml_file, default_flow_style=False)
