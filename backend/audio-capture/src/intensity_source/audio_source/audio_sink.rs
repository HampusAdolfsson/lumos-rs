use std::collections::VecDeque;
use std::collections::vec_deque::Drain;


/// An audio sample buffer.
///
/// This struct can be used to continously collect small audio data buffers into larger buffers of a predetermined size.
pub struct AudioSink {
    /// The number of samples to return in each buffer.
    output_size: usize,
    /// Received audio data that has not yet been returned.
    buffer: VecDeque<f32>,
}

impl AudioSink {
    /// Creates a new audio sink.
    ///
    /// `output_size` - The size of the buffers (iterators) to return from [Self::receive_samples].
    pub fn new(output_size: usize) -> Self {
        AudioSink {
            output_size,
            buffer: VecDeque::with_capacity(2*output_size),
        }
    }


    /// Gets the size of the buffers (iterators) returned from [Self::receive_samples].
    pub fn size(&self) -> usize {
        self.output_size
    }

    /// Stores samples in the internal buffer, returning an iterator if enough samples have been buffered.
    ///
    /// This function will collect samples, returning [None], until it has received [Self::output_buffer_size] samples.
    /// At that point it returns an iterator for the first [Self::output_buffer_size] samples received, and then removes
    /// those samples. Any remaining samples count towards the next iterator.
    pub fn receive_samples(&mut self, samples: &[f32]) -> Option<Drain<f32>> {
        // Since the buffer only drains when this fuction is called, and it can only drain self.output_buffer_size
        // samples at a time, repeatedly adding more samples than that in a single call will fill up the buffer faster
        // than it is drained
        if samples.len() > self.output_size {
            log::warn!("AudioSink: Adding more samples at once than supported. This can lead to memory and latency buildup.");
        }
        self.buffer.extend(samples.iter());
        if self.buffer.len() >= self.output_size {
            let data = self.buffer.drain(0..self.output_size);
            return Some(data);
        }
        None
    }
}
