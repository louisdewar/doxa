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

    print("STARTUP")

    try:
        # get the input and output directory paths from DOXA
        input_path = Path(sys.argv[1])
        output_path = Path(sys.argv[2])
    except IndexError:
        raise Exception(
            f"Run using: {sys.argv[0]} [input directory] [output directory]"
        )

    # process input group files
    while True:
        msg = input()
        if not msg.startswith("Process "):
            raise ValueError(f"Unknown messsage {msg}")

        checkpoint_path = msg[8:]
        group_data = np.load(input_path / checkpoint_path)

        osgb = group_data["osgb"]
        data = group_data["data"]

        # predict future satellite imagery for each array of 12 images
        with torch.no_grad():
            predictions = []
            try:
                for j in range(data.shape[0]):
                    prediction = model(
                        torch.from_numpy(data[j]).view(-1, 12 * 128 * 128)
                    )
                    predictions.append(prediction.view(12, 64, 64).detach().numpy())
            except Exception as err:
                raise Exception(f"Error while processing {checkpoint_path}: {str(err)}")

            # save the group output
            np.savez(
                output_path / checkpoint_path,
                data=np.stack(predictions),
            )
            print(f"Exported {checkpoint_path}")


if __name__ == "__main__":
    main()
