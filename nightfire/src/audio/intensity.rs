use std::collections::HashMap;
use std::vec::Vec;
use crate::audio::FilterID;

/// A value that slowly fades down after getting pushed up.  The value also normalizes itself between 0 and 1.
/// The value itself decays with an exponential decay, while the max value decays linearly.
#[derive(Copy, Clone)]
pub struct NormalizedDecayingValue {
    base_value: f32,
    max_value: f32,
    pub decay_factor: f32,
    decay_value_for_normal_max: f32, // per second
    decayed_time: f32,               // in seconds
}

impl NormalizedDecayingValue {
    pub fn new(decay_factor: f32, decay_value_for_normal_max: f32) -> NormalizedDecayingValue {
        NormalizedDecayingValue {
            base_value: 0.,
            max_value: 0.,
            decay_factor: decay_factor,
            decay_value_for_normal_max: decay_value_for_normal_max,
            decayed_time: 0.,
        }
    }

    fn current_value(&self) -> f32 {
        // TODO make decay a spline, not a hardcoded function
        self.base_value * self.decay_factor.powf(self.decayed_time)
    }

    /// Update with a new, unnormalized value, and the time passed since the last update.
    pub fn update(&mut self, new_value: f32, time_delta: f32) -> f32 {
        let current_max =
            self.max_value - self.decay_value_for_normal_max * (self.decayed_time + time_delta);
        let new_max = current_max.max(new_value);
        let normalized_new_value = new_value / new_max;
        if normalized_new_value > self.current_value() {
            self.max_value = new_max;
            self.base_value = normalized_new_value;
            self.decayed_time = 0.;
        } else {
            self.decayed_time += time_delta;
        }
        self.current_value()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct IntensityID(pub String);

impl IntensityID {
    pub fn get(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Clone)]
pub enum IntensityInputParams {
    TakeMax(Vec<FilterID>)
}

fn get_pre_intensity_value(input_params: &IntensityInputParams, filter_vals: &HashMap<FilterID, f32>) -> f32 {
    match input_params {
        IntensityInputParams::TakeMax(filter_ids) => {
            let mut res = 0f32;
            for filter_id in filter_ids.iter() {
                res = res.max(*filter_vals.get(&filter_id).unwrap());
            }
            res
        }
    }
}

pub struct NormalizedDecayingParams {
    pub decay_factor: f32,
    pub decay_val_for_max: f32
}

pub struct IntensityParams(pub IntensityInputParams, pub NormalizedDecayingParams);

pub struct IntensityTracker {
    intensity_input_params: IntensityInputParams,
    decay_value: NormalizedDecayingValue
}

impl IntensityTracker {
    pub fn new(input_params: &IntensityInputParams, norm_decay_params: &NormalizedDecayingParams) -> Self {
        Self {
            intensity_input_params: input_params.clone(),
            decay_value: NormalizedDecayingValue::new(norm_decay_params.decay_factor, norm_decay_params.decay_val_for_max)
        }
    }

    pub fn update(&mut self, time_delta: f32, filter_vals: &HashMap<FilterID, f32>) -> f32 {
        let pre_val = get_pre_intensity_value(&self.intensity_input_params, &filter_vals);
        self.decay_value.update(pre_val, time_delta)
    }
}

pub struct IntensityTrackers {
    trackers: HashMap<IntensityID, IntensityTracker>
}

impl IntensityTrackers {
    pub fn new(tracker_params: &HashMap<IntensityID, IntensityParams>) -> Self {
        let mut trackers = HashMap::new();
        for (intensity_id, intensity_params) in tracker_params.iter() {
            trackers.insert(intensity_id.clone(), IntensityTracker::new(&intensity_params.0, &intensity_params.1));
        }
        Self {
            trackers: trackers
        }
    }

    pub fn update(&mut self, time_delta: f32, filter_vals: &HashMap<FilterID, f32>) -> HashMap<IntensityID, f32> {
        let mut res = HashMap::new();
        for (intensity_id, tracker) in self.trackers.iter_mut() {
            res.insert(intensity_id.clone(), tracker.update(time_delta, &filter_vals));
        }
        res
    }
}