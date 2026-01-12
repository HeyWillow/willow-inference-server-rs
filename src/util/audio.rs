use std::io::Cursor;

/// # Errors
/// - when we fail to create `hound::WavWriter`
/// - when we fail to write a sample to the writer
pub fn encode_wav(samples: &[f32], sample_rate: u32) -> anyhow::Result<Vec<u8>> {
    let capacity = samples.len() * 2;

    let wav: Vec<u8> = Vec::with_capacity(capacity);

    let spec = hound::WavSpec {
        bits_per_sample: 16,
        channels: 1,
        sample_format: hound::SampleFormat::Int,
        sample_rate,
    };

    let mut cursor = Cursor::new(wav);
    {
        let mut writer = hound::WavWriter::new(&mut cursor, spec)?;
        for &sample in samples {
            #[allow(clippy::cast_possible_truncation)]
            let sample_i16 = (sample.clamp(-1.0, 1.0) * 32767.0).round() as i16;
            writer.write_sample(sample_i16)?;
        }
        writer.finalize()?;
    }

    Ok(cursor.into_inner())
}
