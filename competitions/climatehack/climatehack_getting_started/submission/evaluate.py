import sys
from pathlib import Path

import numpy as np
import torch

from model import Model


def main():
    # load the trained model (in evaluation mode)
    model = Model()
    model.load_state_dict(torch.load("model.pt"))
    model.eval()

    try:
        # get the input and output directory paths from doxa
        input_path = Path(sys.argv[1])
        output_path = Path(sys.argv[2])

        # get the number of groups
        group_count = int(sys.argv[3])
    except IndexError:
        raise Exception(
            f"Run using: {sys.argv[0]} [input directory] [output directory] [group count]"
        )

    # process input group files
    for i in range(group_count):
        data = np.load(input_path / f"{i}.npz")["data"]

        # predict future satellite imagery for each array of 12 images
        predictions = []
        with torch.no_grad():
            try:
                for j in range(data.shape[0]):
                    prediction = model(
                        torch.from_numpy(data[j]).view(-1, 12 * 128 * 128)
                    )
                    predictions.append(prediction.view(12, 64, 64).detach().numpy())
            except:
                if not predictions:
                    return

            # save the group output
            np.savez(
                output_path / f"{i}.npz",
                data=np.stack(predictions),
            )
            print(f"Exported {i}")


if __name__ == "__main__":
    main()
