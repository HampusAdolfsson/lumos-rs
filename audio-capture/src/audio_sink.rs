use std::collections::VecDeque;


pub struct AudioSink {
    n_channels: usize,
    output_rate_in_frames: usize,
    buffer: VecDeque<f32>,
    sum: f32,
    max_sum: f32,
    prev_vals: VecDeque<f32>
}

impl AudioSink {
    pub fn new(n_channels: usize, output_rate_in_frames: usize) -> Self {
        let mut prev_vals = VecDeque::new();
        prev_vals.extend(vec![0.0; 1]);
        AudioSink {
            n_channels,
            output_rate_in_frames,
            buffer: VecDeque::with_capacity(2*n_channels * output_rate_in_frames),
            sum: 0.0,
            max_sum: 0.0,
            prev_vals,
        }
    }

    pub fn receive_samples(&mut self, samples: &[f32]) -> Option<f32> {
        self.buffer.extend(samples.iter());
        let target_length = self.n_channels * self.output_rate_in_frames;
        if self.buffer.len() >= target_length {
            let data = self.buffer.drain(0..target_length);
            let mut sum: f64 = 0.0;
            let len = data.len();
            for item in data {
                sum += (item * item) as f64;
            }
            let mean = (sum.sqrt() / len as f64) as f32;
            {
                let new_val = mean / self.prev_vals.len() as f32;
                self.sum -= self.prev_vals.pop_front().unwrap();
                self.sum += new_val;
                self.prev_vals.push_back(new_val);
            }
            if self.sum > self.max_sum {
                self.max_sum = self.sum;
            } else {
                // decay max_sum
                if self.max_sum > 0.0000001 {
                    self.max_sum -= (1.0 / 330000.0) / (44100.0 / samples.len() as f32)
                }
            }
            return Some((self.sum / self.max_sum).clamp(0.0, 1.0));
        }
        None
    }
}

struct ExpFilter {
    rise: f32,
    fall: f32,
    output: f32,
}

impl ExpFilter {
    fn new(rise: f32, fall: f32) -> Self {
        ExpFilter { rise, fall, output: 0.0 }
    }

    fn put(&mut self, val: f32) {
        let smoothing = if val < self.output { self.fall } else { self.rise };
        self.output = smoothing * val + (1.0 - smoothing) * self.output;
    }

    fn output(&self) -> f32 {
        self.output
    }
}
