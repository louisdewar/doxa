#!/bin/bash

set -e

# This script runs inside the debian/python docker image during the build process

groupadd --gid 1000 doxa
useradd --uid 1000 --gid 1000 doxa

python -m venv --upgrade-deps /python_env

source "/python_env/bin/activate"

python -m pip install --upgrade pip

python -m pip install numpy scipy pandas scikit-learn numba
python -m pip install tensorflow tf-agents[reverb]
python -m pip install --use-deprecated=html5lib torch==1.10.2+cpu torchvision==0.11.3+cpu torchaudio==0.10.2+cpu -f https://download.pytorch.org/whl/cpu/torch_stable.html
python -m pip install opencv-contrib-python-headless
python -m pip install einops perceiver-pytorch

# Probably not required for evaluation but people may have imported these packages while training and did not separate the logic for evaluation
# python -m pip install matplotlib seaborn
# python -m pip install ipython jupyter nose sympy


python -m pip freeze > /pipfreeze.txt

echo /pipfreeze.txt:
cat /pipfreeze.txt

mkdir /home/doxa
chown -R doxa:doxa /home/doxa

ln -s /scratch/agent /home/doxa/agent
ln -s /scratch/output /output

echo "Done setup"
