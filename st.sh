#!/bin/bash
cd $HOME/app/uxf
tokei -C -slines -tPython,Rust -esetup.py -etarget
unrecognized.py -q
python3 -m flake8 --ignore=W504,W503,E261,E303 .
python3 -m vulture . | grep -v 60%.confidence
# python3 -m vulture py/uxf.py
git st
