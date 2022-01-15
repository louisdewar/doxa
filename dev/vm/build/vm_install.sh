#!/bin/bash

set -e

# This script runs inside the debian/python docker image during the build process

groupadd --gid 1000 doxa
useradd --uid 1000 --gid 1000 doxa

python -m pip install --upgrade pip

pip3 install numpy scipy pandas scikit-learn
pip3 install tensorflow tf-agents[reverb]
pip3 install torch==1.10.1+cpu torchvision==0.11.2+cpu torchaudio==0.10.1+cpu -f https://download.pytorch.org/whl/cpu/torch_stable.html
pip3 install numba

# Probably not required for evaluation but people may have imported these packages while training and did not separate the logic for evaluation
pip3 install matplotlib seaborn
pip3 install ipython jupyter nose sympy


pip3 freeze

mkdir /home/doxa
chown -R doxa:doxa /home/doxa

# Create output dir for competitions that use it
mkdir /output
chown -R doxa:doxa /output

ln -s /scratch/agent /home/doxa/agent
ln -s /usr/local/bin/python /usr/bin/python

echo "Done setup"
