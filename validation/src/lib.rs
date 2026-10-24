pub fn is_valid_gregorian_date(_value: &str) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use serde_json::Value;
    use std::collections::HashSet;

    use super::is_valid_gregorian_date;

    const SCHEMA: &str = include_str!("../../schema/profile.schema.json");
    const PROFILE: &str = include_str!("../../profile.json");

    fn date_regex() -> Regex {
        let schema: Value = serde_json::from_str(SCHEMA).expect("schema JSON parses");
        Regex::new(
            schema
                .pointer("/$defs/date/pattern")
                .and_then(Value::as_str)
                .expect("schema declares a date pattern"),
        )
        .expect("schema date pattern compiles in Rust")
    }

    #[test]
    fn schema_accepts_valid_gregorian_dates_and_rust_check_agrees() {
        let pattern = date_regex();
        for value in [
            "01-31-2026",
            "02-28-2025",
            "02-29-2024",
            "02-29-0400",
            "02-29-2000",
            "02-29-0004",
            "04-30-2026",
            "12-31-9999",
        ] {
            assert!(pattern.is_match(value), "schema should accept {value}");
            assert!(is_valid_gregorian_date(value), "Rust should accept {value}");
        }
    }

    #[test]
    fn schema_rejects_impossible_or_malformed_dates_and_rust_check_agrees() {
        let pattern = date_regex();
        for value in [
            "02-29-2025",
            "02-31-2026",
            "04-31-2026",
            "13-01-2026",
            "00-01-2026",
            "01-00-2026",
            "01-32-2026",
            "02-29-1900",
            "02-29-2100",
            "01-01-0000",
            "2026-01-31",
            " 02-29-2024",
        ] {
            assert!(!pattern.is_match(value), "schema should reject {value}");
            assert!(
                !is_valid_gregorian_date(value),
                "Rust should reject {value}"
            );
        }
    }

    #[test]
    fn every_non_null_profile_date_uses_a_valid_schema_date() {
        let pattern = date_regex();
        let profile: Value = serde_json::from_str(PROFILE).expect("profile JSON parses");
        let fields = HashSet::from(["reviewed_on", "source_date", "last_supported_on"]);
        let mut checked = 0usize;

        fn visit(
            value: &Value,
            path: &str,
            fields: &HashSet<&str>,
            pattern: &Regex,
            checked: &mut usize,
        ) {
            match value {
                Value::Array(items) => {
                    for (index, item) in items.iter().enumerate() {
                        visit(item, &format!("{path}[{index}]"), fields, pattern, checked);
                    }
                }
                Value::Object(object) => {
                    for (key, child) in object {
                        let child_path = format!("{path}.{key}");
                        if fields.contains(key.as_str()) && !child.is_null() {
                            let value = child
                                .as_str()
                                .unwrap_or_else(|| panic!("{child_path} must be a string or null"));
                            assert!(
                                pattern.is_match(value),
                                "{child_path} must be a valid calendar date"
                            );
                            assert!(
                                is_valid_gregorian_date(value),
                                "{child_path} must be a valid Gregorian date"
                            );
                            *checked += 1;
                        }
                        visit(child, &child_path, fields, pattern, checked);
                    }
                }
                _ => {}
            }
        }

        visit(&profile, "$", &fields, &pattern, &mut checked);
        assert!(checked > 0, "profile should contain non-null date fields");
    }
}
