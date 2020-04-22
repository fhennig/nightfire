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


class Sample:
    def __init__(self, hist, beat_grid, info, label):
        self.hist = hist
        self.beat_grid = beat_grid
        self.info = info
        self.label = label


class Song:
    """A full song.  contains the feature sequence, as well as a beat grid
    and some meta information."""
    def __init__(self, raw_dict):
        self.hist = raw_dict['hist']
        self.beat_grid = raw_dict['beat_grid']
        self.info = raw_dict['info']

    @property
    def beat_indices(self) -> List[int]:
        """Returns the indices at which there is a beat."""
        return [i for i in range(len(self.beat_grid)) if self.beat_grid[i]]

    def get_samples(self, indices, label, offset=0, length=1) -> List[Sample]:
        """Returns a list of samples.  For each index, the range if covers is
        calculated by adding the offset to the index and then taking a
        slice of `length`.  If the slice is not fully within the
        available data it is discarded.
        """
        slices = [(i + offset, i + offset + length) for i in indices
                  if i + offset >= 0 and i + offset + length <= len(self.hist)]
        return [Sample(self.hist[a:b], self.beat_grid[a:b], self.info, label)
                for a, b in slices]


class DataDir:
    """Represents a data directory.  The directory is expected to contain
    an 'info.pickle' file.  In there, the names of the data files are
    noted, which should be in the same directory as the 'info.pickle'
    file.
    """
    def __init__(self, directory: str):
        self.directory = directory

    @cached_property
    def _info(self):
        path = os.path.join(self.directory, "info.pickle")
        with open(path, 'rb') as f:
            return pickle.load(f)

    @property
    def params(self):
        return self._info['params']

    @property
    def to_process(self):
        return self._info['to_process']

    @property
    def processed(self):
        return self._info['processed']

    @property
    def failed(self):
        return self._info['failed']

    def get_file(self, filename: str) -> DataFile:
        path = os.path.join(self.directory, filename)
        with open(path, 'rb') as f:
            return Song(pickle.load(f))

