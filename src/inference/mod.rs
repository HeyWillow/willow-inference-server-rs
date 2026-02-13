#[cfg(feature = "stt")]
pub mod stt;
#[cfg(feature = "tts")]
pub mod tts;

#[derive(Debug)]
pub struct InferenceResult<T> {
    pub duration: u64,
    pub output: T,
    pub speedup: f64,
    pub time: f64,
}
