use crate::audio::processors::RunningStats;
use crate::audio::{FilterFreqs, Sample};
use crate::audio::onset::onset_score;
use crate::audio::audio_events::AudioEvent;

pub struct OnsetDetector {
    filter_freqs: FilterFreqs,
    previous_sample: Option<Sample>,
    onset_stats_full: RunningStats,
    onset_stats_bass: RunningStats,
}

impl OnsetDetector {
    pub fn new(filter_freqs: FilterFreqs) -> Self {
        Self {
            filter_freqs: filter_freqs,
            previous_sample: None,
            onset_stats_bass: RunningStats::new(),
            onset_stats_full: RunningStats::new(),
        }
    }

    pub fn update(&mut self, new_sample: &Sample) -> Vec<AudioEvent> {
        let mut events = vec![];
        if let Some(prev_sample) = &self.previous_sample {
            let curr_onset_score = onset_score(&prev_sample.vals, &new_sample.vals);
            self.onset_stats_full.push_val(curr_onset_score);
            if curr_onset_score > self.onset_stats_full.mean + 3. * self.onset_stats_full.mean_dev {
                events.push(AudioEvent::FullOnset(curr_onset_score));
            }

            let curr_bass_onset_score = onset_score(
                &self.filter_freqs.get_bins(130., 700., &prev_sample),
                &self.filter_freqs.get_bins(130., 700., &new_sample),
            );
            self.onset_stats_bass.push_val(curr_bass_onset_score);
            if curr_bass_onset_score > self.onset_stats_bass.mean + 3. * self.onset_stats_bass.mean_dev {
                events.push(AudioEvent::BassOnset(curr_bass_onset_score));
            }
        }
        events
    }
}
