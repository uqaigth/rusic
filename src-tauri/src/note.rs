use std::f32::consts::PI;
use std::time::Duration;
use rodio::source::{Amplify, Source, TakeDuration};

#[derive(Copy, Clone, Debug)]
pub struct Note {
    // 频率
    freq: f32,
    // 采样率
    sample_rate: u32,
    // 响度制控制器
    filter: fn(x: f32) -> f32,
    // 发声时间(s)
    duration: f32,
    num_sample: u32,
}

impl Note {
    #[inline]
    pub fn new(freq: f32, duration: f32) -> Note {
        Note {
            freq,
            sample_rate: 48000,
            filter: |x| if x < 0.8 { 1.0 } else { 0.0 },
            duration,
            num_sample: 0,
        }
    }

    pub fn create(&self) -> Amplify<TakeDuration<Note>> {
        self.take_duration(Duration::from_secs_f32(self.duration))
            .amplify(0.1)
    }
}


impl Iterator for Note {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let value = 2.0 * PI * self.freq * self.num_sample as f32 / self.sample_rate as f32;
        Some(value.sin())
    }
}

impl Source for Note {
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
        self.sample_rate
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

