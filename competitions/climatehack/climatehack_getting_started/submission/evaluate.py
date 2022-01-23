from pathlib import Path

import numpy as np
import torch

from model import Model

# load the trained model
model = Model()
model.load_state_dict(torch.load("model.pt"))
model.eval()

# get the input and output folder paths from DOXA
input_path = Path(input())
output_path = Path(input())

# load the data
data = np.load(input_path)["data"]

with torch.no_grad():
    i = 0
    while i < data.shape[0]:
        # predict the next hour of satellite imagery
        prediction = model(torch.from_numpy(data[i : i + 12]).view(-1, 12 * 128 * 128))

        # output for DOXA to score
        np.savez_compressed(
            output_path / f"{i}.npz", data=prediction.view(12, 64, 64).detach().numpy()
        )

        i += 12
