use std::time::{Duration, SystemTime};

/// A struct that helps to keep track of how long ago the last
/// interaction was.  If it was longer ago than period X, the state
/// switches from active to inactive.
pub struct InactivityTracker {
    t_last_active: SystemTime,
    wait_duration: Duration,
}

impl InactivityTracker {
    pub fn new() -> InactivityTracker {
        InactivityTracker {
            t_last_active: SystemTime::UNIX_EPOCH,
            wait_duration: Duration::from_secs(10),
        }
    }

    /// Register activity.  Resets the inactivity timer.
    pub fn register_activity(&mut self) {
        self.t_last_active = SystemTime::now();
    }

    pub fn is_inactive(&self) -> bool {
        return SystemTime::now().duration_since(self.t_last_active).unwrap() > self.wait_duration;
    }
}
