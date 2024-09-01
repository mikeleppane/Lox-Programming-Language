use thiserror::Error;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Error)]
pub struct LoxError {
    line: usize,
    message: String,
}

impl LoxError {
    #[must_use]
    pub fn error(line: usize, message: &str) -> Self {
        Self {
            line,
            message: message.to_string(),
        }
    }

    pub fn report(&self, location: &str) {
        eprintln!("[line {}] Error {}: {}", self.line, location, self.message);
    }
}

impl std::fmt::Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[line {}] Error: {}", self.line, self.message)
    }
}
