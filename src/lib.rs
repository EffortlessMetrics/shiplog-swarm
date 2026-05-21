//! Domain parsing helpers for ship log record keys.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogKey {
    pub vessel: String,
    pub voyage: u16,
    pub entry: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseLogKeyError {
    MissingSeparator,
    EmptyVessel,
    InvalidVoyage,
    InvalidEntry,
    ExtraSegments,
}

pub fn parse_log_key(input: &str) -> Result<LogKey, ParseLogKeyError> {
    let mut parts = input.split(':');
    let vessel = parts.next().ok_or(ParseLogKeyError::MissingSeparator)?;
    let voyage = parts.next().ok_or(ParseLogKeyError::MissingSeparator)?;
    let entry = parts.next().ok_or(ParseLogKeyError::MissingSeparator)?;

    if parts.next().is_some() {
        return Err(ParseLogKeyError::ExtraSegments);
    }

    if vessel.is_empty() {
        return Err(ParseLogKeyError::EmptyVessel);
    }

    let voyage = parse_voyage(voyage)?;
    let entry = parse_entry(entry)?;

    Ok(LogKey {
        vessel: vessel.to_owned(),
        voyage,
        entry,
    })
}

fn parse_voyage(raw: &str) -> Result<u16, ParseLogKeyError> {
    raw.parse::<u16>()
        .map_err(|_| ParseLogKeyError::InvalidVoyage)
}

fn parse_entry(raw: &str) -> Result<u32, ParseLogKeyError> {
    raw.parse::<u32>()
        .map_err(|_| ParseLogKeyError::InvalidEntry)
}

#[cfg(test)]
mod tests {
    use super::{parse_log_key, LogKey, ParseLogKeyError};

    #[test]
    fn parses_valid_key() {
        let parsed = parse_log_key("atlantic-queen:42:1001");

        assert_eq!(
            parsed,
            Ok(LogKey {
                vessel: "atlantic-queen".to_owned(),
                voyage: 42,
                entry: 1001,
            })
        );
    }

    #[test]
    fn rejects_missing_segments() {
        let parsed = parse_log_key("atlantic-queen:42");

        assert_eq!(parsed, Err(ParseLogKeyError::MissingSeparator));
    }

    #[test]
    fn rejects_empty_vessel() {
        let parsed = parse_log_key(":42:1001");

        assert_eq!(parsed, Err(ParseLogKeyError::EmptyVessel));
    }

    #[test]
    fn rejects_non_numeric_voyage() {
        let parsed = parse_log_key("atlantic-queen:forty-two:1001");

        assert_eq!(parsed, Err(ParseLogKeyError::InvalidVoyage));
    }

    #[test]
    fn rejects_non_numeric_entry() {
        let parsed = parse_log_key("atlantic-queen:42:entry");

        assert_eq!(parsed, Err(ParseLogKeyError::InvalidEntry));
    }

    #[test]
    fn rejects_extra_segments() {
        let parsed = parse_log_key("atlantic-queen:42:1001:extra");

        assert_eq!(parsed, Err(ParseLogKeyError::ExtraSegments));
    }
}
