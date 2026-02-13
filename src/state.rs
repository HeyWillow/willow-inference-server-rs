#[cfg(feature = "stt")]
use crate::inference::stt::SttEngine;
#[cfg(feature = "tts")]
use crate::inference::tts::TtsEngine;

pub struct State {
    #[cfg(feature = "stt")]
    pub stt_engine: Option<SttEngine>,
    #[cfg(feature = "tts")]
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
            #[cfg(feature = "stt")]
            stt_engine: None,
            #[cfg(feature = "tts")]
            tts_engine: None,
        }
    }

    #[must_use]
    #[cfg(feature = "stt")]
    pub fn with_stt_engine(mut self, stt_engine: SttEngine) -> Self {
        self.stt_engine = Some(stt_engine);
        self
    }

    #[must_use]
    #[cfg(feature = "tts")]
    pub fn with_tts_engine(mut self, tts_engine: TtsEngine) -> Self {
        self.tts_engine = Some(tts_engine);
        self
    }
}
