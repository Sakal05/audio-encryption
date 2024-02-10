pub trait Function<T> {
    fn y_function(&self) -> f32;
    fn reverse_y_function(&self) -> f32;
}

#[derive(Debug)]
pub struct KeyParameter<T> {
    pub x: T,
    pub p: f32,
    pub q: f32,
}

impl<T> Function<T> for KeyParameter<T> where T: Into<f32> + Copy {
    fn y_function(&self) -> f32 {
        let x_as_f32: f32 = self.x.into();
        let result = x_as_f32 + self.p + self.q;
        result
    }

    fn reverse_y_function(&self) -> f32 {
        let x_as_f32: f32 = self.x.into();
        let result = x_as_f32 - self.p - self.q;
        result
    }
}

pub fn prevent_overflow(y: f32) -> f32 {
    let y = ((y + (1 as f32)) % (2 as f32)) - (1 as f32);
    y
}

pub fn normalize(n: f32) -> f32 {
    // (n - MIN_ORIGINAL as f32) / (MAX_ORIGINAL as f32 - MIN_ORIGINAL as f32)
    (n - 127.5) / 127.5
    // (n / 127.5) - 1.0
    // let n = (n - 128.0) / 128.0;
    // n
}

pub fn denormalize(n: f32) -> f32 {
    n * 127.5 + 127.5
    // n * (MAX_ORIGINAL as f32 - MIN_ORIGINAL as f32) + MIN_ORIGINAL as f32
    // (n + 1.0) * 127.5
    // let n = (n * 128.0) + 128.0;
    // n
}

pub fn limit_decimal_places(value: f32, decimal_places: u32) -> f32 {
    let multiplier = (10i32).pow(decimal_places) as f32;
    (value * multiplier).round() / multiplier
}

use wavers::Wav;
#[derive(Debug)]
pub struct AudioReadData {
    pub bytes: Vec<f32>,
    pub sample_rate: i32,
    pub n_channels: u16,
}

pub fn read_waver(file_path: &str) -> anyhow::Result<AudioReadData, anyhow::Error> {

    let mut wav: Wav<f32> = Wav::from_path(file_path).expect("File not found");

    let samples: &[f32] = &wav.read().unwrap();
    let sample_rate = wav.sample_rate();
    let n_channels = wav.n_channels();

    Ok(AudioReadData {
        bytes: samples.to_vec(),
        sample_rate,
        n_channels,
    })
}

use wavers::write;
pub fn write_waver(
    file_path: &str,
    bytes: Vec<f32>,
    sample_rate: i32,
    n_channels: u16
) -> anyhow::Result<(), anyhow::Error> {
    let out_fp = file_path;

    write(out_fp, bytes.as_slice(), sample_rate, n_channels).expect("Failed to write data");

    Ok(())
}
