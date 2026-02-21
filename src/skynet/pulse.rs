use crate::Result;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{info, debug, warn};

/// SKYNET Pulse - Heartbeat monitoring system
pub struct Pulse {
    interval: Duration,
    last_pulse: RwLock<Option<Instant>>,
    pulse_count: RwLock<u64>,
    running: RwLock<bool>,
}

impl Pulse {
    /// Create a new Pulse monitor
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            last_pulse: RwLock::new(None),
            pulse_count: RwLock::new(0),
            running: RwLock::new(false),
        }
    }

    /// Create a pulse with default 30-second interval
    pub fn default() -> Self {
        Self::new(Duration::from_secs(30))
    }

    /// Start the pulse monitoring
    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.write().await;
        if *running {
            warn!("Pulse is already running");
            return Ok(());
        }

        *running = true;
        info!("🫀 SKYNET Pulse started (interval: {:?})", self.interval);
        
        let mut last_pulse = self.last_pulse.write().await;
        *last_pulse = Some(Instant::now());
        
        Ok(())
    }

    /// Stop the pulse monitoring
    pub async fn stop(&self) {
        let mut running = self.running.write().await;
        *running = false;
        info!("💔 SKYNET Pulse stopped");
    }

    /// Record a heartbeat
    pub async fn heartbeat(&self) -> Result<()> {
        let running = self.running.read().await;
        if !*running {
            return Err("Pulse is not running".into());
        }

        let now = Instant::now();
        
        // Update last pulse time
        {
            let mut last_pulse = self.last_pulse.write().await;
            *last_pulse = Some(now);
        }

        // Increment pulse count
        {
            let mut count = self.pulse_count.write().await;
            *count += 1;
            debug!("💗 Heartbeat #{}", *count);
        }

        Ok(())
    }

    /// Check if the system is healthy (pulse within expected interval)
    pub async fn is_healthy(&self) -> bool {
        let running = self.running.read().await;
        if !*running {
            return false;
        }

        let last_pulse = self.last_pulse.read().await;
        if let Some(last) = *last_pulse {
            let elapsed = last.elapsed();
            // Consider unhealthy if no pulse for 2x the interval
            elapsed < (self.interval * 2)
        } else {
            false
        }
    }

    /// Get pulse statistics
    pub async fn stats(&self) -> PulseStats {
        let running = *self.running.read().await;
        let count = *self.pulse_count.read().await;
        let last_pulse = *self.last_pulse.read().await;
        
        let time_since_last = last_pulse.map(|t| t.elapsed());
        let is_healthy = self.is_healthy().await;

        PulseStats {
            running,
            pulse_count: count,
            time_since_last,
            interval: self.interval,
            is_healthy,
        }
    }

    /// Run the pulse monitor loop
    pub async fn run(&self) -> Result<()> {
        self.start().await?;

        loop {
            let running = *self.running.read().await;
            if !running {
                break;
            }

            // Send heartbeat
            self.heartbeat().await?;

            // Check health status
            if !self.is_healthy().await {
                warn!("⚠️  SKYNET Pulse health warning");
            }

            // Sleep until next pulse
            tokio::time::sleep(self.interval).await;
        }

        info!("SKYNET Pulse monitor stopped");
        Ok(())
    }
}

/// Pulse statistics
#[derive(Debug, Clone)]
pub struct PulseStats {
    pub running: bool,
    pub pulse_count: u64,
    pub time_since_last: Option<Duration>,
    pub interval: Duration,
    pub is_healthy: bool,
}

impl std::fmt::Display for PulseStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pulse: {} | Count: {} | Last: {:?} | Healthy: {}",
            if self.running { "RUNNING" } else { "STOPPED" },
            self.pulse_count,
            self.time_since_last,
            if self.is_healthy { "YES" } else { "NO" }
        )
    }
}