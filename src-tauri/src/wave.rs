use std::f32::consts::PI;
use std::time::Duration;

use crate::Source;

/// An infinite source that produces a sine.
///
/// Always has a rate of 48kHz and one channel.
#[derive(Clone, Debug)]
pub struct SimpleWave {
    freq: f32,
    num_sample: usize,
}

impl SimpleWave {
    /// The frequency of the sine.
    #[inline]
    pub fn new(freq: f32) -> SimpleWave {
        SimpleWave {
            freq,
            num_sample: 0,
        }
    }
}

impl Iterator for SimpleWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let value = 2.0 * PI * self.freq * self.num_sample as f32 / 48000.0;
        Some(value.sin())
    }
}

impl Source for SimpleWave {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
