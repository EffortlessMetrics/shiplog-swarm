pub mod domain {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ShipLogEntry {
        pub vessel: String,
        pub voyage_id: u32,
        pub status: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ParseEntryError {
        MissingField { expected: &'static str },
        TrailingData,
        InvalidVoyageId { source: String },
    }

    pub fn parse_ship_log_entry(line: &str) -> Result<ShipLogEntry, ParseEntryError> {
        let mut parts = line.split(',').map(str::trim);

        let vessel = next_required(&mut parts, "vessel")?.to_owned();
        let voyage_raw = next_required(&mut parts, "voyage_id")?;
        let status = next_required(&mut parts, "status")?.to_owned();

        if parts.next().is_some() {
            return Err(ParseEntryError::TrailingData);
        }

        let voyage_id =
            voyage_raw
                .parse::<u32>()
                .map_err(|_| ParseEntryError::InvalidVoyageId {
                    source: voyage_raw.to_owned(),
                })?;

        Ok(ShipLogEntry {
            vessel,
            voyage_id,
            status,
        })
    }

    fn next_required<'a>(
        parts: &mut impl Iterator<Item = &'a str>,
        field: &'static str,
    ) -> Result<&'a str, ParseEntryError> {
        let Some(value) = parts.next() else {
            return Err(ParseEntryError::MissingField { expected: field });
        };

        if value.is_empty() {
            return Err(ParseEntryError::MissingField { expected: field });
        }

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::domain::{parse_ship_log_entry, ParseEntryError, ShipLogEntry};

    #[test]
    fn parses_valid_entry() {
        let parsed = parse_ship_log_entry("Aurora,42,Docked");

        assert_eq!(
            parsed,
            Ok(ShipLogEntry {
                vessel: "Aurora".to_owned(),
                voyage_id: 42,
                status: "Docked".to_owned(),
            })
        );
    }

    #[test]
    fn rejects_invalid_voyage_id() {
        let parsed = parse_ship_log_entry("Aurora,forty-two,Docked");

        assert_eq!(
            parsed,
            Err(ParseEntryError::InvalidVoyageId {
                source: "forty-two".to_owned(),
            })
        );
    }

    #[test]
    fn rejects_missing_status() {
        let parsed = parse_ship_log_entry("Aurora,42,");

        assert_eq!(
            parsed,
            Err(ParseEntryError::MissingField { expected: "status" })
        );
    }

    #[test]
    fn rejects_trailing_data() {
        let parsed = parse_ship_log_entry("Aurora,42,Docked,unexpected");

        assert_eq!(parsed, Err(ParseEntryError::TrailingData));
    }
}
