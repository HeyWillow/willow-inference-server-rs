use crate::stt::SttEngine;

pub struct State {
    pub stt_engine: SttEngine,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    #[must_use]
    pub fn new() -> Self {
        let stt_engine = SttEngine::new();

        Self { stt_engine }
    }
}
