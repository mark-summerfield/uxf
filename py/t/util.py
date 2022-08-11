#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import os
import subprocess


def check_server(path=None):
    reply = subprocess.run(['nc', '-vz', 'localhost', '5558'],
                           stdout=subprocess.DEVNULL,
                           stderr=subprocess.DEVNULL)
    if reply.returncode != 0:
        subprocess.Popen(['nohup', './test_server.py'],
                         cwd=path, stdout=subprocess.DEVNULL,
                         stderr=subprocess.DEVNULL,
                         preexec_fn=os.setpgrp).pid
