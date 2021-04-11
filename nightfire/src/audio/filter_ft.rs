use biquad as bq;
use biquad::Biquad;
use std::collections::HashMap;

pub struct BandPassParams {
    pub f_c: f32,
    pub q: f32,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct FilterID(String);

impl FilterID {
    pub fn get(s: &str) -> Self {
        Self(s.to_string())
    }
}

pub enum FilterParams {
    BandPass(BandPassParams),
}

pub fn make_filter(f_s: f32, params: &FilterParams) -> bq::DirectForm2Transposed<f32> {
    match params {
        FilterParams::BandPass(BandPassParams { f_c, q }) => bq::DirectForm2Transposed::<f32>::new(
            bq::Coefficients::<f32>::from_params(
                bq::Type::BandPass,
                bq::Hertz::<f32>::from_hz(f_s).unwrap(),
                bq::Hertz::<f32>::from_hz(*f_c).unwrap(),
                *q,
            )
            .unwrap(),
        ),
    }
}

/// The FilterMap takes definitions of filters as parameters and can subsequently be used to get these filter
/// values from a waveform, which is inputted one sample at a time.
pub struct FilterMap {
    filters: HashMap<FilterID, bq::DirectForm2Transposed<f32>>,
}

impl FilterMap {
    pub fn new(f_s: f32, params: &HashMap<FilterID, FilterParams>) -> Self {
        let mut filters = HashMap::new();
        for (filter_id, filter_params) in params.iter() {
            filters.insert(filter_id.clone(), make_filter(f_s, &filter_params));
        }
        Self { filters: filters }
    }

    pub fn update(&mut self, val: f32) -> HashMap<FilterID, f32> {
        let mut res = HashMap::new();
        for (filter_id, filter) in self.filters.iter_mut() {
            res.insert(filter_id.clone(), filter.run(val));
        }
        res
    }
}

/// The Windower takes signal bins and aggregates them with the "max" functions, with a certain window size.
pub struct Windower {
    window_size: usize,
    missing_samples: usize,
    accumulators: HashMap<FilterID, f32>,
}

impl Windower {
    pub fn new(window_size: usize) -> Self {
        Self {
            window_size: window_size,
            missing_samples: window_size,
            accumulators: HashMap::new(),
        }
    }

    pub fn update(&mut self, vals: HashMap<FilterID, f32>) -> Option<HashMap<FilterID, f32>> {
        for (filter, val) in vals.iter() {
            let v = self.accumulators.entry(filter.clone()).or_insert(0.);
            *v = v.max(*val);
        }
        self.missing_samples -= 1;
        if self.missing_samples == 0 {
            self.missing_samples = self.window_size;
            Some(std::mem::replace(&mut self.accumulators, HashMap::new()))
        } else {
            None
        }
    }
}

/// The FilterFT is a Filter Frequency Transform. It operates on a waveform, typically at 44.1kHz, the sample rate
/// is passed as f_s.  It windows window_size many of these samples together and produces a map from keys to aggregated
/// signals over the window.  The result a map of frequency bins, with an intensity for each frequency bin.
pub struct FilterFT {
    filter_map: FilterMap,
    windower: Windower,
}

impl FilterFT {
    pub fn new(f_s: f32, window_size: usize, params: &HashMap<FilterID, FilterParams>) -> Self {
        Self {
            filter_map: FilterMap::new(f_s, params),
            windower: Windower::new(window_size),
        }
    }

    pub fn update(&mut self, val: f32) -> Option<HashMap<FilterID, f32>> {
        let vals = self.filter_map.update(val);
        self.windower.update(vals)
    }
}
