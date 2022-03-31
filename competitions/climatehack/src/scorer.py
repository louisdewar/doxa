import base64
import io
import json
import sys
from math import log10
from pathlib import Path
from random import randrange

import matplotlib.pyplot as plt
import numpy as np
from matplotlib import colors
from pytorch_msssim import MS_SSIM, SSIM
from torch import from_numpy
from torch.nn import L1Loss, MSELoss


def print_score(score, imgs, metrics, sequences):
    print(
        json.dumps(
            {
                "type": "success",
                "score": score,
                "images": imgs,
                "metrics": metrics,
                "sequences": sequences,
            }
        )
    )


def print_error(error, forfeit=None):
    msg = {"type": "failure", "error": error, "score": 0.0}
    if forfeit:
        msg["forfeit"] = forfeit

    print(json.dumps(msg))


def encode_image(arr, cmap="viridis", norm=None):
    if norm is not None:
        fig = plt.imshow(arr, cmap=cmap, norm=norm)
    else:
        fig = plt.imshow(arr, cmap=cmap)  # vmin=0, vmax=1023

    fig.axes.get_xaxis().set_visible(False)
    fig.axes.get_yaxis().set_visible(False)

    plt.gca().set_axis_off()
    plt.subplots_adjust(top=1, bottom=0, right=1, left=0, hspace=0, wspace=0)

    plt.tight_layout()

    bytes_io = io.BytesIO()
    plt.savefig(bytes_io, format="png", bbox_inches="tight", pad_inches=0, dpi=50)
    bytes_io.seek(0)
    b64_img = base64.b64encode(bytes_io.read())

    return b64_img.decode("utf-8")


def main():
    try:
        group_prediction = Path(sys.argv[1])
        group_true = Path(sys.argv[2])
    except IndexError:
        raise ValueError(
            f"Usage: {sys.argv[0]} [path to the group prediction] [path to file containing the true y values for the group]"
        )

    series = np.load(group_prediction)["data"]
    true = np.load(group_true)["data"]

    if series.dtype != true.dtype:
        print_error(
            f"Bad agent output array type: {series.dtype} instead of {true.shape}.",
            forfeit=f"An output of type {true.dtype} was expected, but {series.dtype} was received.",
        )
        return

    if series.shape != true.shape:
        print_error(
            f"Badly formed agent output arrays: {series.shape} instead of {true.shape}.",
            forfeit=f"An output of shape {true.shape} was expected, but {series.shape} was received.",
        )
        return

    msssim = MS_SSIM(data_range=1023.0, size_average=True, win_size=3, channel=1)
    ssim = SSIM(data_range=1023.0, size_average=True, win_size=3, channel=1)
    mse = MSELoss()
    mae = L1Loss()

    T = series.shape[0]

    metrics = {"msssim": 0.0, "ssim": 0.0, "mse": 0.0, "mae": 0.0, "psnr": 0.0}
    for j in range(T):
        s = from_numpy(series[j]).view(24, 64, 64).unsqueeze(dim=1)
        t = from_numpy(true[j]).view(24, 64, 64).unsqueeze(dim=1)

        metrics["msssim"] += msssim(s, t).item()
        metrics["ssim"] += ssim(s, t).item()
        metrics["mse"] += mse(s, t).item()
        metrics["mae"] += mae(s, t).item()
        metrics["psnr"] += (
            20 * log10(1023.0 / (metrics["mse"] ** 0.5)) if metrics["mse"] != 0 else 0
        )

    print_score(
        score=metrics["msssim"] / T,
        imgs=[
            encode_image(series[randrange(T), randrange(24)]),
            encode_image(series[randrange(T), randrange(24)]),
            encode_image(series[randrange(T), randrange(24)]),
            encode_image(series[randrange(T), randrange(24)]),
            encode_image(series[randrange(T), randrange(24)]),
            encode_image(series[randrange(T), randrange(24)]),
        ],
        metrics={metric: value / T for metric, value in metrics.items()},
        sequences=[
            {
                "pred": [encode_image(img) for img in series[0]],
                "true": [encode_image(img) for img in true[0]],
                "diff": [
                    encode_image(img, cmap="coolwarm", norm=colors.CenteredNorm())
                    for img in (series[0] - true[0])
                ],
            },
        ],
    )


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print_error(str(e))
