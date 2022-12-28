#! /usr/bin/env bash

if [ ! -e .venv ] ; then
    python3 -m venv .venv
    source .venv/bin/activate
    python3 -m pip install \
        -r ./requirements.txt
else
    source .venv/bin/activate
fi

pre-commit install
