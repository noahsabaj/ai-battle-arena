@echo off
echo Testing AI Battle Arena in HEADLESS mode
echo =========================================

REM Backup current config
copy configs\simulation_modes.toml configs\simulation_modes.toml.bak

REM Set to headless mode
echo # Simulation Modes Configuration > configs\simulation_modes.toml
echo [modes] >> configs\simulation_modes.toml
echo default = "headless" >> configs\simulation_modes.toml
echo. >> configs\simulation_modes.toml
echo [visual] >> configs\simulation_modes.toml
echo enable_rendering = true >> configs\simulation_modes.toml
echo enable_ui = true >> configs\simulation_modes.toml
echo vsync = true >> configs\simulation_modes.toml
echo frame_cap = 60 >> configs\simulation_modes.toml
echo. >> configs\simulation_modes.toml
echo [headless] >> configs\simulation_modes.toml
echo enable_rendering = false >> configs\simulation_modes.toml
echo enable_ui = false >> configs\simulation_modes.toml
echo fixed_timestep = true >> configs\simulation_modes.toml
echo timestep_hz = 1000 >> configs\simulation_modes.toml

REM Run in headless mode
cargo run --release

REM Restore original config
copy configs\simulation_modes.toml.bak configs\simulation_modes.toml
del configs\simulation_modes.toml.bak

pause
