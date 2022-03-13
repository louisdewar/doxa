#!/bin/bash

set -e

# This script runs inside the debian/python docker image during the build process

groupadd --gid 1000 doxa
useradd --uid 1000 --gid 1000 doxa

#PYTHON_BIN=$(which python3.9 | /dev/null && which python3.9 || echo "python")

PYTHON_BIN=python3.9

#curl https://bootstrap.pypa.io/get-pip.py -o get-pip.py
#"${PYTHON_BIN}" get-pip.py

"${PYTHON_BIN}" -m venv --upgrade-deps /python_env

source "/python_env/bin/activate"

python -m pip install --upgrade pip

python -m pip install numpy scipy pandas scikit-learn numba
python -m pip install tensorflow tf-agents[reverb] tensorflow-addons[tensorflow]

if [ -z ${GPU+x} ]; then
    echo "Installing GPU packages (cuda)"
    python -m pip install --use-deprecated=html5lib torch==1.11.0+cu113 torchvision==0.12.0+cu113 torchaudio==0.11.0+cu113 -f https://download.pytorch.org/whl/cu113/torch_stable.html
else
    echo "Installing CPU packages"
    python -m pip install --use-deprecated=html5lib torch==1.10.2+cpu torchvision==0.11.3+cpu torchaudio==0.10.2+cpu -f https://download.pytorch.org/whl/cpu/torch_stable.html
fi

python -m pip install opencv-contrib-python-headless fastai
python -m pip install einops perceiver-pytorch pytorch_lightning huggingface_hub antialiased_cnns transformers py7zr

python -m pip freeze > /pipfreeze.txt

echo /pipfreeze.txt:
cat /pipfreeze.txt

mkdir /home/doxa
chown -R doxa:doxa /home/doxa

ln -s /scratch/agent /home/doxa/agent
ln -s /scratch/output /output

echo "Done setup"
