import logging
import sys

import aux
from steps import *
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
    try:
        if 'steps_done' not in cfg: 
            cfg['steps_done'] = 0
    except TypeError:
        cfg = {}
        cfg['steps_done'] = 0
    aux.write_cfg(cfg_file, cfg)

    if cfg['steps_done'] < 1:
        step_1()
        cfg['steps_done'] = 1
    else:
        pfmt("lila", "- REQUIREMENTS already met")
    aux.write_cfg(cfg_file, cfg)

    if cfg['steps_done'] < 2:
        step_2()
        cfg['steps_done'] = 2
    else:
        pfmt("lila", "- Base Image was already burnt")
    aux.write_cfg(cfg_file, cfg)

    if cfg['steps_done'] < 3:
        step_3()
        cfg['steps_done'] = 3
    else:
        pfmt("lila", "- Debian defaults were changed")
    aux.write_cfg(cfg_file, cfg)

    if cfg['steps_done'] < 4:
        step_4()
        cfg['steps_done'] = 4
    else:
        pfmt("lila", "- Raspberry already booted")
    aux.write_cfg(cfg_file, cfg)

    if cfg['steps_done'] < 5:
        step_5()
        eth_ip = input("Enter your IP: ")
        cfg['eth_ip'] = eth_ip
        cfg['steps_done'] = 5
    else:
        pfmt("lila", "- Raspberrys IP is known: " + cfg['eth_ip'])
    aux.write_cfg(cfg_file, cfg)

    if cfg['steps_done'] < 6:
        step_6()
        user = input("Enter the name of the user you want on the Raspberry: ")
        passwd = input("Enter a password for that user: ")
        cfg['user'] = user
        cfg['pass'] = passwd
        cfg['steps_done'] = 6
    else:
        pfmt("lila", "- User (and password) is known: " + cfg['user'])
    aux.write_cfg(cfg_file, cfg)

    aux.write_cfg(cfg_file, cfg)
    # Give us a TEMPORARY password to use



    logger.info("ALL DONE.")


if __name__ == "__main__":
    main()
