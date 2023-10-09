use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex};

type Callback = fn();

pub struct GameTimer {
    interval: Duration,
    callback: Callback,
    is_running: Arc<Mutex<bool>>,
}

impl GameTimer {
    pub fn new(interval_ms: u64, callback: Callback) -> Self {
        let interval = Duration::from_millis(interval_ms);
        GameTimer {
            interval,
            callback,
            is_running: Arc::new(Mutex::new(true)),
        }
    }

    pub fn start(&self) {
        let interval = self.interval;
        let callback = self.callback;
        let is_running = self.is_running.clone();

        thread::spawn(move || {
            let mut last_time = Instant::now();

            while *is_running.lock().unwrap() {
                let now = Instant::now();
                let elapsed = now.duration_since(last_time);

                if elapsed >= interval {
                    last_time = now;
                    (callback)(); // Call the callback function
                }

                // Sleep for a short duration to avoid busy-waiting
                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    pub fn stop(&self) {
        *self.is_running.lock().unwrap() = false;
    }
}
