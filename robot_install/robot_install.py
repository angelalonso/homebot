import logging
import sys
import os

import aux
from steps import \
        step_1, \
        step_2, \
        step_3, \
        step_4, \
        step_5, \
        step_6, \
        step_7, \
        step_8, \
        step_9, \
        step_10, \
        step_11, \
        step_refresh, \
        step_test
from aux import printfmt as pfmt

# TODO next:
#  Make sure no error shows up from having previous keys or their names on cfg.yml
#  Test that ssh worked, y -> remove access from root
#  install git, download this repo
#  install rust, run test


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

    # TODO: Stop at the step that fails first
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
    # Secure access
    cfg = step_7(logger, cfg)
    aux.write_cfg(cfg_file, cfg)
    # Connect to WiFi
    cfg = step_8(logger, cfg)
    aux.write_cfg(cfg_file, cfg)
    # Install required packages
    cfg = step_9(logger, cfg)
    aux.write_cfg(cfg_file, cfg)
    # Git clone of Homebot code
    cfg = step_10(logger, cfg)
    aux.write_cfg(cfg_file, cfg)
    # System service to run the Homebot code
    cfg = step_11(logger, cfg)
    aux.write_cfg(cfg_file, cfg)
    # Refresh code and start/restart the service
    cfg = step_refresh(logger, cfg)
    aux.write_cfg(cfg_file, cfg)

#    cfg = step_test(logger, cfg)

    logger.info("ALL DONE.")


if __name__ == "__main__":
    main()
