pub struct Clock {
    ticks: u128,
    period: f64,
}

impl Clock {
    pub fn new(period: f64) -> Clock {
        Clock {
            ticks: 0,
            period: period,
        }
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
    }

    /// The time in ticks
    pub fn ticks(&self) -> u128 {
        self.ticks
    }

    /// return the current time in milliseconds.
    pub fn time(&self) -> f64 {
        (self.ticks as f64) * self.period
    }
}
