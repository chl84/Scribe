#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewlineMode {
    Lf,
    Crlf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NewlinePolicy {
    preferred_mode: NewlineMode,
    preserve_existing: bool,
}

impl NewlinePolicy {
    pub const fn detected(preferred_mode: NewlineMode) -> Self {
        Self {
            preferred_mode,
            preserve_existing: true,
        }
    }

    pub fn detect(text: &str) -> Self {
        if text.contains("\r\n") {
            Self::detected(NewlineMode::Crlf)
        } else {
            Self::detected(NewlineMode::Lf)
        }
    }

    pub const fn preferred_mode(self) -> NewlineMode {
        self.preferred_mode
    }

    pub const fn preserve_existing(self) -> bool {
        self.preserve_existing
    }
}
