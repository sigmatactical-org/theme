//! Copyright year label shared across Sigma web surfaces.

/// First year the site was published; the lower bound of the displayed copyright range.
pub const COPYRIGHT_START_YEAR: i32 = 2026;

/// Gregorian (proleptic) year for a UNIX timestamp in seconds (UTC).
#[must_use]
fn civil_year_from_unix(secs: i64) -> i32 {
    let days = secs.div_euclid(86_400);
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    y as i32
}

/// Current UTC year, falling back to the start year if the system clock predates the epoch.
#[must_use]
pub fn current_year() -> i32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| civil_year_from_unix(d.as_secs() as i64))
        .unwrap_or(COPYRIGHT_START_YEAR)
}

/// Copyright year label: a single year, or `start–current` once the year rolls over.
#[must_use]
pub fn copyright_years() -> String {
    let current = current_year();
    if current > COPYRIGHT_START_YEAR {
        format!("{COPYRIGHT_START_YEAR}\u{2013}{current}")
    } else {
        COPYRIGHT_START_YEAR.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_year_from_unix_known_dates() {
        assert_eq!(civil_year_from_unix(0), 1970);
        assert_eq!(civil_year_from_unix(1_577_836_800), 2020);
        assert_eq!(civil_year_from_unix(1_609_459_199), 2020);
        assert_eq!(civil_year_from_unix(1_609_459_200), 2021);
    }

    #[test]
    fn current_year_is_at_least_start_year() {
        assert!(current_year() >= COPYRIGHT_START_YEAR);
    }

    #[test]
    fn copyright_years_formats_single_year_or_range() {
        let label = copyright_years();
        assert!(label.starts_with(&COPYRIGHT_START_YEAR.to_string()));
        if current_year() > COPYRIGHT_START_YEAR {
            assert!(label.contains('\u{2013}'));
        } else {
            assert_eq!(label, COPYRIGHT_START_YEAR.to_string());
        }
    }
}
