use std::time::Instant;

pub struct BenchmarkTimer {
    name: String,
    start_time: Instant,
}

impl BenchmarkTimer {
    #[allow(dead_code)]
    pub fn start<S: AsRef<str>>(name: S) -> Self {
        println!("[Benchmark] [START] {}", name.as_ref());

        BenchmarkTimer {
            name: name.as_ref().to_string(),
            start_time: Instant::now(),
        }
    }
}

impl Drop for BenchmarkTimer {
    fn drop(&mut self) {
        println!(
            "[Benchmark] [END]   {} (took {:.3} ms)",
            self.name,
            self.start_time.elapsed().as_secs_f64() * 1000.0
        );
    }
}
