@echo off
echo Running Batch Test - 100 games with 4 parallel threads
echo ================================================

cargo build --release
if %errorlevel% neq 0 exit /b %errorlevel%

cargo run --release -- batch 100 4

pause
