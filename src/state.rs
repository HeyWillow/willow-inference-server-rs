use crate::{stt::SttEngine, tts::TtsEngine};

pub struct State {
    pub stt_engine: Option<SttEngine>,
    pub tts_engine: Option<TtsEngine>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    #[must_use]
    pub fn new() -> Self {
        Self {
            stt_engine: None,
            tts_engine: None,
        }
    }

    #[must_use]
    pub fn with_stt_engine(mut self, stt_engine: SttEngine) -> Self {
        self.stt_engine = Some(stt_engine);
        self
    }

    #[must_use]
    pub fn with_tts_engine(mut self, tts_engine: TtsEngine) -> Self {
        self.tts_engine = Some(tts_engine);
        self
    }
}
