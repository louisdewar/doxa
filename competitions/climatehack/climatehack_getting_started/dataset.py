from datetime import datetime, time, timedelta
from random import randrange
from typing import Iterator, T_co

import numpy as np
import xarray as xr
from numpy import float32
from torch.utils.data import IterableDataset


class ClimateHackDataset(IterableDataset):
    def __init__(
        self,
        dataset: xr.Dataset,
        start_date: datetime = None,
        end_date: datetime = None,
        crops_per_slice: int = 1,
        day_limit: int = 0,
    ) -> None:
        super().__init__()

        self.dataset = dataset
        self.crops_per_slice = crops_per_slice
        self.day_limit = day_limit
        self.cached_items = []

        times = self.dataset.get_index("time")
        self.min_date = times[0].date() if start_date is None else start_date
        self.max_date = times[-1].date() if end_date is None else end_date

        if self.day_limit > 0:
            self.max_date = min(
                self.max_date, self.min_date + timedelta(days=self.day_limit)
            )

    def _image_times(self, start_time, end_time):
        date = self.min_date
        while date < self.max_date:
            current_time = datetime.combine(date, start_time)
            while current_time.time() < end_time:
                yield current_time
                current_time += timedelta(minutes=5)

            date += timedelta(days=1)

    def _get_crop(self, current_time):
        # roughly over the mainland UK
        rand_x = randrange(550, 950 - 128)
        rand_y = randrange(375, 700 - 128)

        # make a data selection
        selection = self.dataset.sel(
            time=slice(
                current_time,
                current_time + timedelta(minutes=55),
            )
        ).isel(
            x=slice(rand_x, rand_x + 128),
            y=slice(rand_y, rand_y + 128),
        )

        # get the OSGB coordinate data
        osgb_data = np.stack(
            [
                selection["x_osgb"].to_numpy().astype(float32),
                selection["y_osgb"].to_numpy().astype(float32),
            ]
        )

        if osgb_data.shape != (2, 128, 128):
            return None

        # get the input satellite imagery
        input_data = selection["data"].to_numpy().astype(float32)

        if input_data.shape != (12, 128, 128):
            return None

        # get the target output
        target_output = (
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

        if target_output.shape != (24, 64, 64):
            return None

        return osgb_data, input_data, target_output

    def __iter__(self) -> Iterator[T_co]:
        if self.cached_items:
            for item in self.cached_items:
                yield item

            return

        start_time = time(9, 0)
        end_time = time(14, 0)

        for current_time in self._image_times(start_time, end_time):
            crops = 0
            while crops < self.crops_per_slice:
                crop = self._get_crop(current_time)
                if crop:
                    self.cached_items.append(crop)
                    yield crop

                crops += 1
