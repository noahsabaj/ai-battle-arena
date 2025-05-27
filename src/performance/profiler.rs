use bevy::prelude::*;
use std::time::{Instant, Duration};
use std::collections::HashMap;

#[derive(Resource)]
pub struct Profiler {
    timings: HashMap<String, Vec<f64>>,
    current_frame: HashMap<String, Instant>,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            timings: HashMap::new(),
            current_frame: HashMap::new(),
        }
    }
    
    pub fn start(&mut self, name: &str) {
        self.current_frame.insert(name.to_string(), Instant::now());
    }
    
    pub fn end(&mut self, name: &str) {
        if let Some(start) = self.current_frame.remove(name) {
            let duration = start.elapsed().as_secs_f64() * 1000.0; // Convert to ms
            self.timings.entry(name.to_string())
                .or_insert_with(Vec::new)
                .push(duration);
        }
    }
    
    pub fn report(&mut self) {
        println!("\n========== PROFILER REPORT ==========");
        for (name, timings) in &self.timings {
            if !timings.is_empty() {
                let avg = timings.iter().sum::<f64>() / timings.len() as f64;
                let min = timings.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                let max = timings.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                println!("{}: avg={:.3}ms, min={:.3}ms, max={:.3}ms", name, avg, min, max);
            }
        }
        println!("=====================================\n");
        
        // Clear timings after report
        self.timings.clear();
    }
}

pub struct ProfilerPlugin;

impl Plugin for ProfilerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Profiler::new())
            .add_systems(Update, report_profiler.run_if(on_timer(Duration::from_secs(10))));
    }
}

fn report_profiler(mut profiler: ResMut<Profiler>) {
    profiler.report();
}

fn on_timer(duration: Duration) -> impl FnMut(Local<Option<Instant>>) -> bool {
    move |mut last_run: Local<Option<Instant>>| {
        let now = Instant::now();
        let should_run = last_run
            .map(|last| now.duration_since(last) >= duration)
            .unwrap_or(true);
        
        if should_run {
            *last_run = Some(now);
        }
        
        should_run
    }
}
