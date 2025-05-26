use std::time::Instant;

pub struct Metrics {
    pub start_time: Instant,
    pub tick_count: u64,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            tick_count: 0,
        }
    }
    
    pub fn ticks_per_second(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            self.tick_count as f64 / elapsed
        } else {
            0.0
        }
    }
}
