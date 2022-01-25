from datetime import datetime, time, timedelta
from random import randrange
from typing import Iterator, T_co

import numpy as np
from numpy import float32
from torch.utils.data import IterableDataset


class ClimateHackDataset(IterableDataset):
    def __init__(self, dataset, samples_per_slice=1, day_limit=0, cache=True) -> None:
        super().__init__()

        self.dataset = dataset
        self.samples_per_slice = samples_per_slice
        self.day_limit = day_limit
        self.cache = True
        self.cached_items = []

    def __iter__(self) -> Iterator[T_co]:
        if self.cached_items:
            for item in self.cached_items:
                yield item

            return

        times = self.dataset.get_index("time")
        start_date = times[0].date()
        end_date = times[-1].date()
        end_time = time(14, 0)

        if self.day_limit > 0:
            end_date = min(end_date, start_date + timedelta(days=self.day_limit))

        date = start_date
        while date < end_date:
            current_time = datetime.combine(date, time(9, 0))
            while current_time.time() < end_time:
                for _ in range(self.samples_per_slice):
                    rand_x = randrange(550, 950 - 128)
                    rand_y = randrange(375, 700 - 128)

                    selection = self.dataset.sel(
                        time=slice(
                            current_time,
                            current_time + timedelta(minutes=55),
                        )
                    ).isel(
                        x=slice(rand_x, rand_x + 128),
                        y=slice(rand_y, rand_y + 128),
                    )

                    osgb_data = np.stack(
                        [
                            selection["x_osgb"].to_numpy().astype(float32),
                            selection["y_osgb"].to_numpy().astype(float32),
                        ]
                    )

                    input_data = selection["data"].to_numpy().astype(float32)

                    true_output = (
                        self.dataset["data"]
                        .sel(
                            time=slice(
                                current_time + timedelta(hours=1),
                                current_time + timedelta(hours=2, minutes=55),
                            )
                        )
                        .isel(
                            x=slice(rand_x + 32, rand_x + 96),
                            y=slice(rand_y + 32, rand_y + 96),
                        )
                        .to_numpy()
                        .astype(float32)
                    )

                    res = (osgb_data, input_data, true_output)
                    if self.cache:
                        self.cached_items.append(res)

                    yield res

                current_time += timedelta(minutes=5)

            date += timedelta(days=1)
