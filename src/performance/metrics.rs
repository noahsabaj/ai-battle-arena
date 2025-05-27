use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use std::time::Instant;
use crate::config::GameConfig;

#[derive(Resource)]
pub struct PerformanceMetrics {
    pub start_time: Instant,
    pub tick_count: u64,
    pub frame_count: u64,
    pub last_report: Instant,
    pub target_tps: u32,
    pub actual_tps: f64,
    pub actual_fps: f64,
}

impl PerformanceMetrics {
    pub fn new(target_tps: u32) -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            tick_count: 0,
            frame_count: 0,
            last_report: now,
            target_tps,
            actual_tps: 0.0,
            actual_fps: 0.0,
        }
    }
    
    pub fn record_tick(&mut self) {
        self.tick_count += 1;
    }
    
    pub fn record_frame(&mut self) {
        self.frame_count += 1;
    }
    
    pub fn calculate_rates(&mut self) -> (f64, f64) {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            self.actual_tps = self.tick_count as f64 / elapsed;
            self.actual_fps = self.frame_count as f64 / elapsed;
            (self.actual_tps, self.actual_fps)
        } else {
            (0.0, 0.0)
        }
    }
    
    pub fn print_final_report(&mut self) {
        let (tps, fps) = self.calculate_rates();
        let elapsed = self.start_time.elapsed().as_secs_f64();
        
        println!("\n========== FINAL PERFORMANCE REPORT ==========");
        println!("Total Runtime: {:.2}s", elapsed);
        println!("Target TPS: {}", self.target_tps);
        println!("Achieved TPS: {:.2}", tps);
        println!("Achieved FPS: {:.2}", fps);
        println!("Total Game Ticks: {}", self.tick_count);
        println!("Total Frames: {}", self.frame_count);
        
        let efficiency = (tps / self.target_tps as f64) * 100.0;
        println!("Efficiency: {:.1}% of target", efficiency);
        
        if tps < self.target_tps as f64 * 0.9 {
            println!("⚠️  Performance below target!");
        } else {
            println!("✅ Performance target achieved!");
        }
        println!("==============================================\n");
    }
}

pub struct MetricsPlugin;

impl Plugin for MetricsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        
        // Get config after plugins are added
        app.add_systems(Startup, setup_metrics);
        app.add_systems(Update, (
            update_metrics,
            report_metrics.run_if(should_report_metrics),
        ));
    }
}

fn setup_metrics(
    mut commands: Commands,
    config: Res<GameConfig>,
) {
    commands.insert_resource(PerformanceMetrics::new(config.performance.target_tps));
}

fn update_metrics(
    mut metrics: ResMut<PerformanceMetrics>,
) {
    metrics.record_frame();
}

fn should_report_metrics(
    metrics: Res<PerformanceMetrics>,
    config: Res<GameConfig>,
) -> bool {
    config.performance.enable_metrics && 
    metrics.last_report.elapsed().as_secs_f32() >= config.performance.metrics_interval
}

fn report_metrics(
    mut metrics: ResMut<PerformanceMetrics>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let (tps, fps) = metrics.calculate_rates();
    
    println!("\n========== PERFORMANCE METRICS ==========");
    println!("Target TPS: {}", metrics.target_tps);
    println!("Actual TPS: {:.2}", tps);
    println!("Actual FPS: {:.2}", fps);
    println!("Total Ticks: {}", metrics.tick_count);
    println!("Total Frames: {}", metrics.frame_count);
    println!("Uptime: {:.1}s", metrics.start_time.elapsed().as_secs_f64());
    
    if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_avg) = fps_diagnostic.average() {
            println!("Bevy FPS (avg): {:.2}", fps_avg);
        }
    }
    
    println!("=========================================\n");
    
    metrics.last_report = Instant::now();
}
