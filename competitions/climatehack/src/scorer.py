import json
import sys
from pathlib import Path

import numpy as np
from pytorch_msssim import MS_SSIM
from torch import from_numpy


def print_score(score):
    print(json.dumps({"score": score}))


def print_error(error):
    print(json.dumps({"error": error, "score": 0.0}))


def main():
    try:
        group_prediction = Path(sys.argv[1])
        group_true = Path(sys.argv[2])
    except IndexError:
        print(
            f"Usage: {sys.argv[0]} [path to the group prediction] [path to file containing the true y values for the group]"
        )
        return

    series = np.load(group_prediction)["data"]
    true = np.load(group_true)["data"]

    assert series.shape == true.shape

    losses = 0
    for j in range(series.shape[0]):
        criterion = MS_SSIM(data_range=1024.0, size_average=True, win_size=3, channel=1)

        loss = criterion(
            from_numpy(series[j]).view(24, 64, 64).unsqueeze(dim=1),
            from_numpy(true[j]).view(24, 64, 64).unsqueeze(dim=1),
        ).item()

        losses += loss

    print_score(losses / series.shape[0])


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print_error(str(e))
