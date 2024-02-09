pub trait Function<T> {
    fn y_function(&self) -> f32;
    fn reverse_y_function(&self) -> f32;
}

#[derive(Debug)]
pub struct KeyParameter<T> {
    pub x: T,
    pub p: f32,
    pub q: f32
}


impl<T> Function<T> for KeyParameter<T> where
T: Into<f32> + Copy,
{
    fn y_function(&self) -> f32 {
        let x_as_f32: f32 = self.x.into(); // Convert character to a numeric type first
        // print all value
        // println!("x: {}, p: {}, q: {}", x_as_f32, self.p, self.q);
        let result = x_as_f32 + self.p + self.q; // Perform the arithmetic and implicitly return the result
        // println!("F Function result: {}", result);
        result
    }

    fn reverse_y_function(&self) -> f32 {
        let x_as_f32: f32 = self.x.into(); // Convert character to a numeric type first
        // print all value
        // println!("x: {}, p: {}, q: {}", x_as_f32, self.p, self.q);
        let result = x_as_f32 - self.p - self.q; // Perform the arithmetic and implicitly return the result
        // println!("F Function result: {}", result);
        result
    }
}

pub fn prevent_overflow(y: &mut f32) {
    *y = (*y + 1.00 % 2.00) - 1.00;
    // *y = y.clamp(0.0, 1.0);

}

pub fn normalize(n:f32) -> f32 {
    // (n - MIN_ORIGINAL as f32) / (MAX_ORIGINAL as f32 - MIN_ORIGINAL as f32)
    (n - 127.5) / 127.5
    // (n / 127.5) - 1.0
    // let n = (n - 128.0) / 128.0;
    // n
}

pub fn denormalize(n: f32) -> f32 {
    (n * 127.5) + 127.5
    // n * (MAX_ORIGINAL as f32 - MIN_ORIGINAL as f32) + MIN_ORIGINAL as f32
    // (n + 1.0) * 127.5
    // let n = (n * 128.0) + 128.0;
    // n
}

use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};

pub fn read_file(file_path: &str) -> Vec<u8> {
    let file = File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut buffer).unwrap();

    buffer
}

pub fn write_file(file_path: &str, bytes: Vec<u8>){
    let mut file =  BufWriter::new(File::create(file_path).unwrap());
    file.write_all(&bytes).unwrap();
}

pub fn limit_decimal_places(value: f32, decimal_places: usize) -> f32 {
    let multiplier = 10u64.pow(decimal_places as u32) as f32;
    (value * multiplier).round() / multiplier
}


use wavers::Wav;

pub struct AudioReadData {
    pub bytes: Vec<f32>,
    pub sample_rate: i32,
    pub n_channels: u16
}

pub fn read_waver(file_path: &str) -> AudioReadData {
    // Two ways of converted a wav file
    // let fp: "./path/to/i16_encoded_wav.wav";
    let mut wav: Wav<f32> = Wav::from_path(file_path).unwrap();
    // conversion happens automatically when you read
    let samples: &[f32] = &wav.read().unwrap();
    let sample_rate = wav.sample_rate();
    let n_channels = wav.n_channels();

    // or read and then call the convert function on the samples.
    // let (samples, sample_rate): (Samples<i16>, i32) = read::<i16, _>(file_path).unwrap();
    // let samples: &[f32] = &samples.convert();
    // println!("Sample file data: {:?}", samples);
    AudioReadData {
        bytes: samples.to_vec(),
        sample_rate,
        n_channels,
    }
    
}

use wavers::write;
pub fn write_waver(file_path: &str, bytes: Vec<f32>, sample_rate: i32, n_channels: u16) {
	// let fp: &Path = &Path::new("path/to/wav.wav");
	let out_fp = file_path;

	// two main ways, read and write as the type when reading
    // let wav: Wav<i16> = Wav::from_path(fp).unwrap();
    // wav.write(out_fp).unwrap();

	// or read, convert, and write
    // let (samples, sample_rate): (Samples<i16>,i32) = read::<i16, _>(fp).unwrap();
    // let sample_rate = wav.sample_rate();
    // let n_channels = wav.n_channels();

    // let samples: &[f32] = &samples.convert();
    write(out_fp, bytes.as_slice(), sample_rate, n_channels).unwrap();
}

// Function to encode audio samples into MP3 format and write to a file
// pub fn write_mp3(samples: &[f32], sample_rate: u32, channels: usize, output_path: &str) -> io::Result<()> {
//     // Convert f32 samples to i16 samples
//     let samples_i16: Vec<i16> = samples.iter().map(|&x| (x * i16::MAX as f32) as i16).collect();

//     // Open the output file
//     let mut file = File::create(output_path)?;

//     // Create an MP3 encoder
//     let mut encoder = Encoder::new(sample_rate, channels as u16)?;

//     // Write the MP3 header to the file
//     file.write_all(&encoder.get_header())?;

//     // Encode audio samples and write to the file
//     let mut mp3_buf = Vec::new();
//     for chunk in samples_i16.chunks(channels) {
//         let mp3_data = encoder.encode(chunk)?;
//         mp3_buf.extend_from_slice(&mp3_data);
//     }

//     // Write the encoded MP3 data to the file
//     file.write_all(&mp3_buf)?;

//     Ok(())
// }

// use mp3lame_encoder::{Builder, Encoder, Id3Tag, DualPcm, FlushNoGap};
// pub fn write_mp3() {

//     let mut mp3_encoder = Builder::new().expect("Create LAME builder");
//     // Encoder::encode();
//     // let _mp3lame_encoder::encoder();
//     mp3_encoder.set_num_channels(2).expect("set channels");
//     mp3_encoder.set_sample_rate(44_100).expect("set sample rate");
//     mp3_encoder.set_brate(mp3lame_encoder::Bitrate::Kbps192).expect("set brate");
//     mp3_encoder.set_quality(mp3lame_encoder::Quality::Best).expect("set quality");
//     mp3_encoder.set_id3_tag(Id3Tag {
//         title: b"Mix hah",
//         artist: &[],
//         album: b"My album",
//         year: b"2025",
//         comment: b"Just my comment",
//     });
//     let mut mp3_encoder = mp3_encoder.build().expect("To initialize LAME encoder");

//     //use actual PCM data
//     let input = DualPcm {
//         left: &[0u16, 0],
//         right: &[0u16, 0],
//     };

//     let mut mp3_out_buffer: Vec<u8> = Vec::new();
//     mp3_out_buffer.reserve(mp3lame_encoder::max_required_buffer_size(input.left.len()));
//     let encoded_size = mp3_encoder.encode(input, mp3_out_buffer.spare_capacity_mut()).expect("To encode");
//     unsafe {
//         mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
//     }

//     let encoded_size = mp3_encoder.flush::<FlushNoGap>(mp3_out_buffer.spare_capacity_mut()).expect("to flush");
//     unsafe {
//         mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
//     }
//     //At this point your mp3_out_buffer should have full MP3 data, ready to be written on file system or whatever

// }

// use symphonia::core::audio::SampleBuffer;
// use symphonia::core::codecs::DecoderOptions;
// use symphonia::core::conv::IntoSample;
// use symphonia::core::errors::Error;
// use symphonia::core::formats::FormatOptions;
// use symphonia::core::io::MediaSourceStream;
// use symphonia::core::meta::MetadataOptions;
// use symphonia::core::probe::Hint;

// pub fn read_mp3(path: &str) -> Option<Vec<f32>> {
//     // let path = args.get(1).expect("file path not provided");

//     // Open the media source.
//     let file = Box::new(File::open(path).unwrap());

//     // Create the media source stream using the boxed media source from above.
//     let mss = MediaSourceStream::new(file, Default::default());

//     // Create a hint to help the format registry guess what format reader is appropriate. In this
//     // example we'll leave it empty.
//     let hint = Hint::new();

//     // Use the default options when reading and decoding.
//     let format_opts: FormatOptions = Default::default();
//     let metadata_opts: MetadataOptions = Default::default();
//     let decoder_opts: DecoderOptions = Default::default();

//     // Probe the media source stream for a format.
//     let probed =
//         symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts).unwrap();

//     // Get the format reader yielded by the probe operation.
//     let mut format = probed.format;

//     // Get the default track.
//     let track = format.default_track().unwrap();

//     // Create a decoder for the track.
//     let mut decoder =
//         symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts).unwrap();

//     // Store the track identifier, we'll use it to filter packets.
//     let track_id = track.id;

//     let mut sample_count = 0;
//     let mut sample_buf = None;

//     loop {
//         // Get the next packet from the format reader.
//         // let packet = format.next_packet().ok()?;
//         let packet = match format.next_packet() {
//             Ok(packet) => packet,
//             Error => break, // No more packets, exit loop
//         };

//         // If the packet does not belong to the selected track, skip it.
//         if packet.track_id() != track_id {
//             continue;
//         }

//         // Decode the packet into audio samples, ignoring any decode errors.
//         match decoder.decode(&packet) {
//             Ok(audio_buf) => {
//                 // The decoded audio samples may now be accessed via the audio buffer if per-channel
//                 // slices of samples in their native decoded format is desired. Use-cases where
//                 // the samples need to be accessed in an interleaved order or converted into
//                 // another sample format, or a byte buffer is required, are covered by copying the
//                 // audio buffer into a sample buffer or raw sample buffer, respectively. In the
//                 // example below, we will copy the audio buffer into a sample buffer in an
//                 // interleaved order while also converting to a f32 sample format.

//                 // If this is the *first* decoded packet, create a sample buffer matching the
//                 // decoded audio buffer format.
//                 if sample_buf.is_none() {
//                     // Get the audio buffer specification.
//                     let spec = *audio_buf.spec();

//                     // Get the capacity of the decoded buffer. Note: This is capacity, not length!
//                     let duration = audio_buf.capacity() as u64;

//                     // Create the f32 sample buffer.
//                     sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
//                 }

//                 // Copy the decoded audio buffer into the sample buffer in an interleaved format.
//                 if let Some(buf) = &mut sample_buf {
//                     buf.copy_interleaved_ref(audio_buf);
//                     // println!("Buffer: {:?} ", buf.samples());
//                     // The samples may now be access via the `samples()` function.
//                     sample_count += buf.samples().len();
//                     print!("\rDecoded {} samples", sample_count);
//                 }
//             }
//             Err(Error::DecodeError(_)) => (),
//             Err(_) => break,
//         }
//     }

//     if let Some(sample_buf) = sample_buf {
//         Some(sample_buf.samples().to_vec())
//     } else {
//         None
//     }
// }

