use crate::stt::SttEngine;

pub struct State {
    pub stt_engine: Option<SttEngine>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    #[must_use]
    pub fn new() -> Self {
        Self { stt_engine: None }
    }

    #[must_use]
    pub fn with_stt_engine(mut self, stt_engine: SttEngine) -> Self {
        self.stt_engine = Some(stt_engine);
        self
    }
}
