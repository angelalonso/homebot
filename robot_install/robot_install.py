import logging
import sys
import os

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

    # Use config file to store status
    cfg_file = "cfg.yml"
    try:
        cfg = aux.read_cfg(cfg_file)
    except FileNotFoundError:
        os.mknod(cfg_file)
        cfg = {}

    # Guide user through preparations
    # We avoid repeating steps that were done
    try:
        if 'steps_done' not in cfg:
            cfg['steps_done'] = 0
    except TypeError:
        cfg = {}
        cfg['steps_done'] = 0
    aux.write_cfg(cfg_file, cfg)

    # Check steps.py for what each one does
    # Requirements
    cfg = step_1(cfg)
    aux.write_cfg(cfg_file, cfg)
    # Image burn
    cfg = step_2(cfg)
    aux.write_cfg(cfg_file, cfg)
    # Modify base system 
    cfg = step_3(cfg)
    aux.write_cfg(cfg_file, cfg)
    # Boot Raspberry
    cfg = step_4(cfg)
    aux.write_cfg(cfg_file, cfg)
    # Add Raspberry IP
    cfg = step_5(cfg, cfg_file)
    aux.write_cfg(cfg_file, cfg)

    cfg = aux.get_sshkeypair(logger, cfg)
    # Create user
    cfg = step_6(logger, cfg)
    aux.write_cfg(cfg_file, cfg)


    cfg = step_test(logger, cfg)

    logger.info("ALL DONE.")


if __name__ == "__main__":
    main()
