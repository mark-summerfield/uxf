#!/bin/bash
cd $HOME/app/uxf
tokei -C -f -slines -tPython -esetup.py -eeg -emisc -ex/ -epy/t \
    -eregression.py -echeck_lexer.py
tokei -C -f -tRust -etarget -e rs/tests -e x/rs \
    | grep -v '^-- ' | grep -v '|-'
unrecognized.py -q
python3 -m flake8 --ignore=W504,W503,E261,E303 .
python3 -m vulture . | grep -v 60%.confidence
# python3 -m vulture py/uxf.py
git st
