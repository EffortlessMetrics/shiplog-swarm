#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShiplogEntry {
    pub vessel: String,
    pub port: String,
    pub eta_hour: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseEntryError {
    MissingField(&'static str),
    InvalidEtaHour,
    ExtraFields,
}

pub fn parse_shiplog_entry(input: &str) -> Result<ShiplogEntry, ParseEntryError> {
    let mut fields = input.split(',').map(str::trim);

    let vessel = next_required(&mut fields, "vessel")?;
    let port = next_required(&mut fields, "port")?;
    let eta_raw = next_required(&mut fields, "eta_hour")?;

    if fields.next().is_some() {
        return Err(ParseEntryError::ExtraFields);
    }

    let eta_hour = parse_eta_hour(eta_raw)?;

    Ok(ShiplogEntry {
        vessel: vessel.to_owned(),
        port: port.to_owned(),
        eta_hour,
    })
}

fn next_required<'a>(
    fields: &mut impl Iterator<Item = &'a str>,
    name: &'static str,
) -> Result<&'a str, ParseEntryError> {
    let value = fields.next().ok_or(ParseEntryError::MissingField(name))?;

    if value.is_empty() {
        return Err(ParseEntryError::MissingField(name));
    }

    Ok(value)
}

fn parse_eta_hour(raw: &str) -> Result<u8, ParseEntryError> {
    let hour = raw
        .parse::<u8>()
        .map_err(|_| ParseEntryError::InvalidEtaHour)?;

    if hour <= 23 {
        Ok(hour)
    } else {
        Err(ParseEntryError::InvalidEtaHour)
    }
}

#[cfg(test)]
mod tests {
    use super::{ParseEntryError, ShiplogEntry, parse_shiplog_entry};

    #[test]
    fn parses_valid_entry() {
        let parsed = parse_shiplog_entry("Aurora, Seattle, 17");

        assert_eq!(
            parsed,
            Ok(ShiplogEntry {
                vessel: "Aurora".to_string(),
                port: "Seattle".to_string(),
                eta_hour: 17,
            })
        );
    }

    #[test]
    fn rejects_missing_field() {
        let parsed = parse_shiplog_entry("Aurora, , 17");

        assert_eq!(parsed, Err(ParseEntryError::MissingField("port")));
    }

    #[test]
    fn rejects_invalid_eta_hour() {
        let parsed = parse_shiplog_entry("Aurora, Seattle, 24");

        assert_eq!(parsed, Err(ParseEntryError::InvalidEtaHour));
    }

    #[test]
    fn rejects_extra_fields() {
        let parsed = parse_shiplog_entry("Aurora, Seattle, 17, delayed");

        assert_eq!(parsed, Err(ParseEntryError::ExtraFields));
    }
}
