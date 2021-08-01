use std::{
    borrow::Cow,
    error::Error,
    fmt::{
        Display,
        Formatter,
    },
};

pub type Result<T = ()> = std::result::Result<T, TestError>;

#[derive(Debug)]
pub struct TestError {
    pub test: Cow<'static, str>,
    pub location: &'static str,
    pub reason: &'static str,
    pub message: Option<Cow<'static, str>>,
}

impl Display for TestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let separator = if f.alternate() { "\n" } else { " " };
        write!(
            f,
            "Test '{}' failed.{nl}Location: {}.{nl}Reason: {}.{nl}",
            self.test,
            self.location,
            self.reason,
            nl = separator
        )?;

        if let Some(message) = self.message {
            writeln!(f, "Message: {}.", message)?;
        } else if !f.alternate() {
            writeln!(f)
        }

        Ok(())
    }
}

impl Error for TestError {}
