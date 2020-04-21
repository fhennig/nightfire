"""In here data loading funtions are defined that load preprocessed
songs for training.  The data is created and its format specified by
the mixxx_data program.

The data set is inside a given directory.  The directory contains an
info.pickle file which holds further information on where to find the
files.

The info.pickle also contains information about the parameters used to
generate the data.

The individual files contain the title of the song, the original file,
the bpm, the processed spectogram as well as a target vector of
matching length, annotating the location of beats.
"""
import os.path
import pickle
from dataclasses import dataclass
from functools import cached_property
from typing import List


class DataSetInfo:
    f_low: float
    f_high: float
    q: float
    n_filters: int
    rate: float
    files: List[str]

    def __init__(self, attr_vals):
        for key in attr_vals:
            setattr(self, key, attr_vals[key])


class DataFile:
    title: str
    bpm: float
    original_file: str
    hist: List[List[float]]
    target: List[bool]

    def __init__(self, attr_vals):
        for key in attr_vals:
            setattr(self, key, attr_vals[key])


class DataDir:
    """Represents a data directory.  The directory is expected to contain
    an 'info.pickle' file.  In there, the names of the data files are
    noted, which should be in the same directory as the 'info.pickle'
    file.
    """
    def __init__(self, directory: str):
        self.directory = directory

    @cached_property
    def info(self):
        path = os.path.join(self.directory, "info.pickle")
        with open(path, 'rb') as f:
            data = dict(pickle.load(f))
        return DataSetInfo(data)

    def get_file(self, filename: str) -> DataFile:
        path = os.path.join(self.directory, filename)
        with open(path, 'rb') as f:
            d = dict(pickle.load(f))
            return DataFile(d)

    def __iter__(self):
        for filename in self.info.files:
            yield self.get_file(filename)
