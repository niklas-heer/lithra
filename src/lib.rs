//! Reproducible systems and images, described in a language you can read.
//!
//! Lithra is an early experiment. The library holds the logic, and the
//! `lithra` binary is a thin command-line layer on top.

/// Returns the line printed by `lithra --version`.
///
/// ```
/// assert_eq!(lithra::version_line(), format!("lithra {}", env!("CARGO_PKG_VERSION")));
/// ```
#[must_use]
pub fn version_line() -> String {
    format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[cfg(test)]
mod tests {
    use super::version_line;

    #[test]
    fn version_line_names_the_binary() {
        assert!(version_line().starts_with("lithra "));
    }
}
