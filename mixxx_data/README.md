# Mixxx data loader

A small binary that loads tracks with beat grids from
[Mixxx](https://www.mixxx.org/), and creates training data for beat
detection in the python
[`pickle`](https://docs.python.org/3/library/pickle.html) format.

## Running

The program takes the following arguments, all of which are required
except for the `database` parameter:

        --database <DB_FILE>           Mixxx DB location (default: ~/.mixxx/mixxxdb.sqlite)
    -h, --high <F_HIGH>                highest frequency to capture.
    -l, --low <F_LOW>                  lowest frequency to capture.
    -k, --num_filters <NUM_FILTERS>    number of frequency bands to capture.
    -t, --threads <NUM_THREADS>        the number of threads to use.  Defaults to using as many as makes sense on the
                                       CPU.
    -o, --output_dir <OUTPUT_DIR>      the directory to put the results in.
    -q <Q>                             the q parameter for the filters.
    -r, --rate <RATE>                  subsampling rate in Hz.


## Output File Structure

The output structure looks like this:

    output_dir/
      info.pickle
      track_a.pickle
      another_track.pickle
      ...

Each pickle file is a list of tuples, which should be made into a
dictionary with string keys.

The format for the `info.pickle` file is:

    f_low: float
    f_high: float
    q: float
    n_filters: int
    rate: float
    files: List[str]

The first five values are parameters used in the feature extraction
process.
    
The `files` key contains a list of file names, which should be like
`[track_a.pickle, another_track.pickle, ...]`, i.e. give the file
names of the track files in the directory.

The format of a track file is:

    title: str
    bpm: float
    original_file: str
    hist: List[List[float]]
    target: List[bool]

The `hist` key contains an NxD matrix with N samples and feature
dimension D.  The `target` contains a matching list of length N, which
represents the beat grid.

## Details

The data is in `~/.mixxx/mixxxdb.sqlite`.  In the `beats` column there
is a protobuf structure which contains the bpm as a float and the
offset in integers for the beat onset.
