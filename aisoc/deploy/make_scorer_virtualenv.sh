#!/bin/bash

python -m venv --upgrade-deps --copies /scorer_env

source "/scorer_env/bin/activate"

python -m pip install --upgrade pip

python -m pip install numpy pytorch-msssim matplotlib
python -m pip install torch==1.10.1+cpu torchvision==0.11.2+cpu -f https://download.pytorch.org/whl/cpu/torch_stable.html

