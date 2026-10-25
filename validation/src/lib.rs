use regex::Regex;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
    sync::OnceLock,
};

pub fn is_valid_gregorian_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 10
        || bytes[2] != b'-'
        || bytes[5] != b'-'
        || !bytes[..2]
            .iter()
            .chain(bytes[3..5].iter())
            .chain(bytes[6..].iter())
            .all(u8::is_ascii_digit)
    {
        return false;
    }

    let month = usize::from(bytes[0] - b'0') * 10 + usize::from(bytes[1] - b'0');
    let day = usize::from(bytes[3] - b'0') * 10 + usize::from(bytes[4] - b'0');
    let year = u32::from(bytes[6] - b'0') * 1000
        + u32::from(bytes[7] - b'0') * 100
        + u32::from(bytes[8] - b'0') * 10
        + u32::from(bytes[9] - b'0');

    if year == 0 || !(1..=12).contains(&month) || day == 0 {
        return false;
    }

    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return false,
    };
    day <= days_in_month
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PrivacyFinding {
    category: String,
    source: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationSummary {
    pub records: usize,
    pub groups: usize,
    pub sources: usize,
    pub files: usize,
}

fn validate_dates(value: &Value, path: &str, findings: &mut Vec<String>) {
    const DATE_FIELDS: [&str; 3] = ["reviewed_on", "source_date", "last_supported_on"];
    match value {
        Value::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                validate_dates(child, &format!("{path}[{index}]"), findings);
            }
        }
        Value::Object(object) => {
            for (key, child) in object {
                let child_path = format!("{path}.{key}");
                if DATE_FIELDS.contains(&key.as_str()) && !child.is_null() {
                    match child.as_str() {
                        Some(date) if is_valid_gregorian_date(date) => {}
                        _ => findings.push(format!("invalid Gregorian date at {child_path}")),
                    }
                }
                validate_dates(child, &child_path, findings);
            }
        }
        _ => {}
    }
}

fn validate_profile_structure(profile: &Value) -> Vec<String> {
    let mut findings = Vec::new();
    validate_dates(profile, "$", &mut findings);

    let mut group_ids = HashSet::new();
    if let Some(groups) = profile.get("groups").and_then(Value::as_array) {
        for group in groups {
            if let Some(id) = group.get("id").and_then(Value::as_str) {
                if !group_ids.insert(id.to_owned()) {
                    findings.push("duplicate group id".to_string());
                }
            }
        }
    }

    let mut source_ids = HashSet::new();
    if let Some(sources) = profile.get("sources").and_then(Value::as_array) {
        for source in sources {
            if let Some(id) = source.get("id").and_then(Value::as_str) {
                if !source_ids.insert(id.to_owned()) {
                    findings.push("duplicate source id".to_string());
                }
            }
        }
    }

    let mut record_ids = HashSet::new();
    if let Some(records) = profile.get("records").and_then(Value::as_array) {
        for record in records {
            if let Some(id) = record.get("id").and_then(Value::as_str) {
                if !record_ids.insert(id.to_owned()) {
                    findings.push("duplicate record id".to_string());
                }
            }

            if let Some(group) = record.get("group").and_then(Value::as_str) {
                if !group_ids.contains(group) {
                    findings.push("record references unknown group".to_string());
                }
            }

            if let Some(ids) = record.get("source_ids").and_then(Value::as_array) {
                let mut seen = HashSet::new();
                for source in ids.iter().filter_map(Value::as_str) {
                    if !seen.insert(source) {
                        findings.push("duplicate record source reference".to_string());
                    }
                    if !source_ids.contains(source) {
                        findings.push("record references unknown source".to_string());
                    }
                }
            }
        }
    }

    findings
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn verify_manifest_entries(manifest: &Value, files: &HashMap<String, Vec<u8>>) -> Vec<String> {
    let mut findings = Vec::new();
    let Some(entries) = manifest.get("files").and_then(Value::as_array) else {
        return vec!["manifest has no file inventory".to_string()];
    };

    let mut listed = HashSet::new();
    for entry in entries {
        let Some(path) = entry.get("path").and_then(Value::as_str) else {
            findings.push("manifest has an invalid file path".to_string());
            continue;
        };
        if path == "manifest.json" {
            findings.push("manifest must not list itself".to_string());
        }
        if !listed.insert(path.to_owned()) {
            findings.push(format!("duplicate manifest path: {path}"));
            continue;
        }

        let Some(expected_bytes) = entry.get("bytes").and_then(Value::as_u64) else {
            findings.push(format!("manifest has invalid byte count: {path}"));
            continue;
        };
        let Some(expected_hash) = entry.get("sha256").and_then(Value::as_str) else {
            findings.push(format!("manifest has invalid SHA-256: {path}"));
            continue;
        };
        match files.get(path) {
            Some(actual) => {
                if actual.len() as u64 != expected_bytes {
                    findings.push(format!("byte count mismatch: {path}"));
                }
                if sha256_hex(actual) != expected_hash {
                    findings.push(format!("SHA-256 mismatch: {path}"));
                }
            }
            None => findings.push(format!("missing manifest file: {path}")),
        }
    }

    for path in files.keys() {
        if !listed.contains(path) {
            findings.push(format!("unlisted file: {path}"));
        }
    }

    findings
}

fn validate_rendered_records(content: &str, records: &Value, selected_ids: &[&str]) -> Vec<String> {
    let mut findings = Vec::new();
    let Some(records) = records.as_array() else {
        return vec!["profile records are not an array".to_string()];
    };

    let mut seen = HashSet::new();
    let mut record_text = HashMap::new();
    for record in records {
        if let (Some(id), Some(text)) = (
            record.get("id").and_then(Value::as_str),
            record.get("text").and_then(Value::as_str),
        ) {
            record_text.insert(id, text);
        }
    }

    for id in selected_ids {
        if !seen.insert(*id) {
            findings.push("render selection contains a duplicate record".to_string());
            continue;
        }
        let Some(text) = record_text.get(id) else {
            findings.push("render selection references an unknown record".to_string());
            continue;
        };

        let anchor = format!("<a id=\"record-{id}\"></a>");
        let line = format!("- **{id}** — {text}");
        let anchor_position = content.find(&anchor);
        let line_position = content.find(&line);
        let exactly_once =
            content.matches(&anchor).count() == 1 && content.matches(&line).count() == 1;
        let contiguous = match (anchor_position, line_position) {
            (Some(anchor_position), Some(line_position))
                if line_position >= anchor_position + anchor.len() =>
            {
                content[anchor_position + anchor.len()..line_position]
                    .trim()
                    .is_empty()
            }
            _ => false,
        };
        if !exactly_once || !contiguous {
            findings.push(format!("rendered record is missing or duplicated: {id}"));
        }
    }

    let actual_ids: Vec<&str> = content
        .lines()
        .filter_map(|line| {
            line.strip_prefix("<a id=\"record-")
                .and_then(|rest| rest.strip_suffix("\"></a>"))
        })
        .collect();
    if actual_ids != selected_ids {
        findings.push(
            "rendered record selection is incomplete, duplicated, or out of order".to_string(),
        );
    }

    findings
}

fn collect_markdown_anchors(content: &str) -> HashSet<String> {
    let mut anchors = HashSet::new();
    let explicit = Regex::new(r#"(?i)<a\s+id=["']([^"']+)["']"#).expect("static anchor regex");
    for capture in explicit.captures_iter(content) {
        if let Some(id) = capture.get(1) {
            anchors.insert(id.as_str().to_owned());
        }
    }

    let mut slug_counts: HashMap<String, usize> = HashMap::new();
    for line in content.lines() {
        let heading = line.trim_start();
        let level = heading
            .chars()
            .take_while(|character| *character == '#')
            .count();
        if !(1..=6).contains(&level)
            || !heading
                .chars()
                .nth(level)
                .map(char::is_whitespace)
                .unwrap_or(false)
        {
            continue;
        }

        let mut title = heading[level..]
            .trim()
            .trim_end_matches('#')
            .trim()
            .to_lowercase();
        title = title
            .chars()
            .filter(|character| {
                character.is_alphanumeric() || *character == '-' || character.is_whitespace()
            })
            .collect::<String>();
        let slug = title.split_whitespace().collect::<Vec<_>>().join("-");
        if slug.is_empty() {
            continue;
        }
        let count = slug_counts.entry(slug.clone()).or_insert(0);
        let final_slug = if *count == 0 {
            slug
        } else {
            format!("{slug}-{}", *count)
        };
        *count += 1;
        anchors.insert(final_slug);
    }

    anchors
}

fn normalize_markdown_links(content: &str) -> String {
    Regex::new(r"\[([^\]]+)\]\([^)]+\)")
        .expect("static link regex")
        .replace_all(content, "[$1]")
        .into_owned()
}

fn validate_markdown_links(root: &Path, source: &Path, content: &str) -> Vec<String> {
    let mut findings = Vec::new();
    let links = Regex::new(r"!?\[[^\]]*\]\(([^)]+)\)").expect("static Markdown link regex");
    let source_text = source.to_string_lossy().replace('\\', "/");

    for capture in links.captures_iter(content) {
        let Some(raw) = capture.get(1).map(|item| item.as_str().trim()) else {
            continue;
        };
        let target = if raw.starts_with('<') {
            raw.split('>').next().unwrap_or(raw).trim_start_matches('<')
        } else {
            raw.split_whitespace().next().unwrap_or(raw)
        };
        if target.is_empty()
            || target.starts_with("//")
            || Regex::new(r"^[A-Za-z][A-Za-z0-9+.-]*:")
                .expect("static URL regex")
                .is_match(target)
        {
            continue;
        }

        let (path_part, fragment) = target.split_once('#').unwrap_or((target, ""));
        let path_part = path_part.split('?').next().unwrap_or(path_part);
        let target_path = if path_part.is_empty() {
            source.to_path_buf()
        } else {
            let path = Path::new(path_part);
            if path.is_absolute() {
                findings.push(format!("broken local link in {source_text}"));
                continue;
            }
            source.parent().unwrap_or(Path::new("")).join(path)
        };

        let root_canonical = match root.canonicalize() {
            Ok(path) => path,
            Err(_) => {
                findings.push("repository root is unavailable during link validation".to_string());
                return findings;
            }
        };
        let candidate = root.join(&target_path);
        let resolved = match candidate.canonicalize() {
            Ok(path) => path,
            Err(_) => {
                findings.push(format!("broken local link in {source_text}"));
                continue;
            }
        };
        if !resolved.starts_with(&root_canonical) {
            findings.push(format!("broken local link in {source_text}"));
            continue;
        }

        let resolved = if resolved.is_dir() && !fragment.is_empty() {
            let readme = resolved.join("README.md");
            if readme.is_file() {
                readme
            } else {
                findings.push(format!("broken local anchor in {source_text}"));
                continue;
            }
        } else {
            resolved
        };

        if !fragment.is_empty() {
            let linked_content = match fs::read_to_string(&resolved) {
                Ok(text) => text,
                Err(_) => {
                    findings.push(format!("broken local anchor in {source_text}"));
                    continue;
                }
            };
            if !collect_markdown_anchors(&linked_content).contains(fragment) {
                findings.push(format!("broken local anchor in {source_text}"));
            }
        }
    }

    findings
}

fn privacy_patterns() -> &'static [(String, Regex)] {
    static PATTERNS: OnceLock<Vec<(String, Regex)>> = OnceLock::new();
    PATTERNS.get_or_init(|| {
        [
            (
                "credential",
                r"(?i)\b(?:ghp|gho|ghu|ghs|ghr|github_pat)_[a-z0-9_]{20,}\b",
            ),
            ("credential", r"(?i)\bsk-[a-z0-9_-]{20,}\b"),
            ("credential", r"\bAKIA[0-9A-Z]{16}\b"),
            ("credential", r"(?i)\bxox[baprs]-[a-z0-9-]{20,}\b"),
            (
                "credential",
                r#"(?i)\b(?:api[_-]?key|access[_-]?token|client[_-]?secret|password|passwd)\s*[:=]\s*["']?[a-z0-9_./+=-]{12,}"#,
            ),
            (
                "email",
                r"(?i)\b[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}\b",
            ),
            (
                "phone",
                r"\b(?:\+?1[ .-]?)?(?:\([2-9][0-9]{2}\)|[2-9][0-9]{2})[ .-]+[2-9][0-9]{2}[ .-]+[0-9]{4}\b",
            ),
            (
                "phone",
                r"(?i)\b(?:phone|mobile|telephone|tel)\s*[:=]\s*[0-9]{10,11}\b",
            ),
            (
                "street address",
                r"(?im)^\s*(?:address|home address|mailing address|street address)\s*[:=]\s*[0-9]{1,6}\s+(?:[a-z0-9.'#-]+\s+){1,4}(?:street|st\.?|road|rd\.?|avenue|ave\.?|drive|dr\.?|lane|ln\.?|court|ct\.?|boulevard|blvd\.?|way|parkway|pkwy\.?)\b[^\n]*$",
            ),
            (
                "street address",
                r#"(?i)"(?:home|mailing|street)?_?address"\s*:\s*"\s*[0-9]{1,6}\s+(?:[a-z0-9.'#-]+\s+){1,4}(?:street|st\.?|road|rd\.?|avenue|ave\.?|drive|dr\.?|lane|ln\.?|court|ct\.?|boulevard|blvd\.?|way|parkway|pkwy\.?)\b"#,
            ),
            (
                "street address",
                r"(?im)^\s*[0-9]{1,6}\s+(?:[a-z0-9.'#-]+\s+){1,4}(?:street|st\.?|road|rd\.?|avenue|ave\.?|drive|dr\.?|lane|ln\.?|court|ct\.?|boulevard|blvd\.?|way|parkway|pkwy\.?)\s*$",
            ),
            ("government identifier", r"\b[0-9]{3}-[0-9]{2}-[0-9]{4}\b"),
            ("payment card", r"\b(?:[0-9][ -]?){13,19}\b"),
            (
                "account number",
                r"(?i)\baccount(?:\s*(?:number|no\.?|#))\s*[:=]\s*[0-9][0-9 -]{5,}\b",
            ),
            (
                "account number",
                r"(?i)\baccount(?:\s*(?:number|no\.?|#))\s*[:=]\s*[0-9][0-9 -]{5,}\b",
            ),
            (
                "raw transcript",
                r"(?im)^\s*(?:begin (?:private )?chat|raw (?:chat|conversation) (?:export|transcript)|chat transcript)\s*$",
            ),
            (
                "raw transcript",
                r"(?im)^(?:[ \t]*(?:user|assistant|system|developer|human|chatgpt|claude|gemini)[ \t]*:[ \t]*[^\n]+\n){3,}",
            ),
        ]
        .into_iter()
        .map(|(category, pattern)| {
            (
                category.to_string(),
                Regex::new(pattern).expect("static privacy regex"),
            )
        })
        .collect()
    })
}

fn is_safe_fixture(category: &str, value: &str, source: &str) -> bool {
    let path = source.rsplit(':').next().unwrap_or(source);
    if path == "LICENSE" && ["email", "street address"].contains(&category) {
        return true;
    }
    if path != "validation/src/lib.rs" {
        return false;
    }

    match category {
        "email" => ["validator@example.invalid", "person@fixture.invalid"]
            .contains(&value.to_ascii_lowercase().as_str()),
        "phone" => value == "202-555-0123",
        "street address" => value.eq_ignore_ascii_case("123 Test Street"),
        "payment card" => value == "4111111111111111",
        _ => false,
    }
}

fn scan_privacy_text(content: &str, source: &str) -> Vec<PrivacyFinding> {
    let mut findings = Vec::new();
    for (category, pattern) in privacy_patterns() {
        let has_match = pattern.find_iter(content).any(|matched| {
            let embedded_phone = category == "phone"
                && ((matched.start() > 0
                    && content.as_bytes()[matched.start() - 1].is_ascii_digit())
                    || content
                        .as_bytes()
                        .get(matched.end())
                        .map(u8::is_ascii_digit)
                        .unwrap_or(false));
            !embedded_phone && !is_safe_fixture(category, matched.as_str(), source)
        });
        if has_match {
            findings.push(PrivacyFinding {
                category: category.clone(),
                source: source.to_owned(),
            });
        }
    }
    findings
}

fn git_output(root: &Path, arguments: &[&str]) -> Option<Vec<u8>> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(root)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    output.status.success().then_some(output.stdout)
}

fn scan_batch_objects(
    root: &Path,
    objects: &[(String, String)],
    fallback_source: &str,
) -> Vec<PrivacyFinding> {
    if objects.is_empty() {
        return Vec::new();
    }

    let object_ids: Vec<String> = objects.iter().map(|(id, _)| id.clone()).collect();
    let mut child = match Command::new("git")
        .args(["cat-file", "--batch"])
        .current_dir(root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(child) => child,
        Err(_) => {
            return vec![PrivacyFinding {
                category: "git-scan-unavailable".to_string(),
                source: fallback_source.to_string(),
            }];
        }
    };

    if let Some(mut input) = child.stdin.take() {
        for id in &object_ids {
            if input.write_all(id.as_bytes()).is_err() || input.write_all(b"\n").is_err() {
                return vec![PrivacyFinding {
                    category: "git-scan-unavailable".to_string(),
                    source: fallback_source.to_string(),
                }];
            }
        }
    }
    let output = match child.wait_with_output() {
        Ok(output) if output.status.success() => output.stdout,
        _ => {
            return vec![PrivacyFinding {
                category: "git-scan-unavailable".to_string(),
                source: fallback_source.to_string(),
            }];
        }
    };

    let mut findings = Vec::new();
    let mut cursor = 0usize;
    let mut object_index = 0usize;
    while cursor < output.len() {
        let Some(relative_end) = output[cursor..].iter().position(|byte| *byte == b'\n') else {
            break;
        };
        let header_end = cursor + relative_end;
        let header = String::from_utf8_lossy(&output[cursor..header_end]);
        cursor = header_end + 1;

        let mut parts = header.split_whitespace();
        let _object_id = parts.next();
        let object_type = parts.next();
        let size = parts.next().and_then(|value| value.parse::<usize>().ok());
        let source = objects
            .get(object_index)
            .map(|(_, source)| source.as_str())
            .unwrap_or(fallback_source);
        object_index += 1;

        let (Some(object_type), Some(size)) = (object_type, size) else {
            continue;
        };
        let end = cursor.saturating_add(size);
        if end > output.len() {
            break;
        }
        if object_type == "blob" {
            let text = String::from_utf8_lossy(&output[cursor..end]);
            findings.extend(scan_privacy_text(&text, source));
        }
        cursor = end;
        if output.get(cursor) == Some(&b'\n') {
            cursor += 1;
        }
    }

    findings
}

fn scan_git_privacy(root: &Path) -> Vec<PrivacyFinding> {
    let mut findings = Vec::new();
    let Some(paths) = git_output(
        root,
        &[
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
            "-z",
        ],
    ) else {
        return vec![PrivacyFinding {
            category: "git-scan-unavailable".to_string(),
            source: "worktree".to_string(),
        }];
    };

    for raw_path in paths
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
    {
        let path = String::from_utf8_lossy(raw_path).into_owned();
        if let Ok(bytes) = fs::read(root.join(&path)) {
            let text = String::from_utf8_lossy(&bytes);
            findings.extend(scan_privacy_text(&text, &format!("worktree:{path}")));
        }
    }

    if let Some(index) = git_output(root, &["ls-files", "--stage", "-z"]) {
        let staged_objects: Vec<(String, String)> = index
            .split(|byte| *byte == 0)
            .filter_map(|entry| {
                let separator = entry.iter().position(|byte| *byte == b'\t')?;
                let metadata = String::from_utf8_lossy(&entry[..separator]);
                let path = String::from_utf8_lossy(&entry[separator + 1..]);
                let object_id = metadata.split_whitespace().nth(1)?;
                Some((object_id.to_owned(), format!("staged:{path}")))
            })
            .collect();
        findings.extend(scan_batch_objects(root, &staged_objects, "staged:index"));
    } else {
        findings.push(PrivacyFinding {
            category: "git-scan-unavailable".to_string(),
            source: "staged:index".to_string(),
        });
    }

    if let Some(history) = git_output(root, &["rev-list", "--objects", "--all"]) {
        let history_objects: Vec<(String, String)> = history
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                let separator = line.iter().position(|byte| *byte == b' ')?;
                let object_id = String::from_utf8_lossy(&line[..separator]);
                let path = String::from_utf8_lossy(&line[separator + 1..]);
                Some((object_id.into_owned(), format!("history:{path}")))
            })
            .collect();
        findings.extend(scan_batch_objects(
            root,
            &history_objects,
            "history:reachable",
        ));
    } else {
        findings.push(PrivacyFinding {
            category: "git-scan-unavailable".to_string(),
            source: "history:reachable".to_string(),
        });
    }

    let mut unique = HashSet::new();
    findings.retain(|finding| unique.insert((finding.category.clone(), finding.source.clone())));
    findings
}

fn schema_uses_only_local_references(schema: &Value) -> bool {
    fn visit(value: &Value) -> bool {
        match value {
            Value::Array(items) => items.iter().all(visit),
            Value::Object(object) => object.iter().all(|(key, child)| {
                if ["$ref", "$dynamicRef", "$recursiveRef"].contains(&key.as_str()) {
                    child
                        .as_str()
                        .map(|reference| reference.starts_with('#'))
                        .unwrap_or(false)
                } else {
                    visit(child)
                }
            }),
            _ => true,
        }
    }
    visit(schema)
}

fn parse_json_file(bytes: &[u8], path: &str) -> Result<Value, String> {
    serde_json::from_slice(bytes).map_err(|_| format!("invalid JSON: {path}"))
}

fn git_tracked_and_unignored_paths(root: &Path) -> Result<Vec<String>, String> {
    let paths = git_output(
        root,
        &[
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
            "-z",
        ],
    )
    .ok_or_else(|| "unable to enumerate repository files".to_string())?;
    Ok(paths
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| String::from_utf8_lossy(path).into_owned())
        .collect())
}

fn expected_record_ids_for(
    profile_groups: &[Value],
    records: &Value,
    selected_groups: &HashSet<String>,
    selected_records: Option<&HashSet<String>>,
) -> Vec<String> {
    let Some(records) = records.as_array() else {
        return Vec::new();
    };
    let mut selected_ids = Vec::new();

    for group in profile_groups {
        let Some(group_id) = group.get("id").and_then(Value::as_str) else {
            continue;
        };
        if !selected_groups.contains(group_id) {
            continue;
        }
        for record in records {
            let Some(record_id) = record.get("id").and_then(Value::as_str) else {
                continue;
            };
            if record.get("group").and_then(Value::as_str) == Some(group_id)
                && selected_records
                    .map(|explicit| explicit.contains(record_id))
                    .unwrap_or(true)
            {
                selected_ids.push(record_id.to_owned());
            }
        }
    }

    selected_ids
}

fn validate_required_render_outputs(
    outputs: &[Value],
    group_paths: &HashMap<String, String>,
    output_paths: &HashSet<String>,
) -> Vec<String> {
    let complete_paths: HashSet<String> = outputs
        .iter()
        .filter(|output| output.get("mode").and_then(Value::as_str) == Some("complete_with_contract"))
        .filter_map(|output| output.get("path").and_then(Value::as_str).map(str::to_owned))
        .collect();
    let required_complete_paths = HashSet::from([
        "START_HERE.md".to_string(),
        "exports/agent-instructions.md".to_string(),
    ]);
    let mut findings = Vec::new();
    if complete_paths != required_complete_paths {
        findings.push(
            "render specification must include both required complete exports".to_string(),
        );
    }

    let compact_paths: HashSet<String> = outputs
        .iter()
        .filter(|output| output.get("mode").and_then(Value::as_str) == Some("selected_records"))
        .filter_map(|output| output.get("path").and_then(Value::as_str).map(str::to_owned))
        .collect();
    if compact_paths != HashSet::from(["exports/compact-profile.md".to_string()]) {
        findings.push("render specification must include the required compact export".to_string());
    }

    let mut required_paths: HashSet<String> = group_paths.values().cloned().collect();
    required_paths.extend([
        "START_HERE.md".to_string(),
        "exports/agent-instructions.md".to_string(),
        "exports/compact-profile.md".to_string(),
    ]);
    if output_paths != &required_paths {
        findings.push("render specification does not cover the exact required output paths".to_string());
    }

    findings
}

fn validate_render_spec(
    spec: &Value,
    profile: &Value,
    files: &HashMap<String, Vec<u8>>,
) -> Vec<String> {
    let mut findings = Vec::new();
    let records = profile.get("records").unwrap_or(&Value::Null);
    let profile_groups = profile
        .get("groups")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let group_ids: HashSet<String> = profile_groups
        .iter()
        .filter_map(|group| group.get("id").and_then(Value::as_str).map(str::to_owned))
        .collect();
    let mut group_paths = HashMap::new();
    for group in &profile_groups {
        if let (Some(id), Some(path)) = (
            group.get("id").and_then(Value::as_str),
            group.get("path").and_then(Value::as_str),
        ) {
            group_paths.insert(id.to_owned(), path.to_owned());
        }
    }

    let Some(outputs) = spec.get("outputs").and_then(Value::as_array) else {
        return vec!["render specification has no outputs".to_string()];
    };
    if spec.get("canonical").and_then(Value::as_str) != Some("profile.json")
        || spec.get("version") != profile.get("version")
        || spec.get("reviewed_on") != profile.get("reviewed_on")
    {
        findings.push("render specification metadata is stale".to_string());
    }

    let mut output_paths = HashSet::new();
    let mut detailed_group_counts: HashMap<String, usize> = HashMap::new();
    let mut complete_paths = Vec::new();

    for output in outputs {
        let Some(path) = output.get("path").and_then(Value::as_str) else {
            findings.push("render output has no path".to_string());
            continue;
        };
        if !output_paths.insert(path.to_owned()) {
            findings.push(format!("duplicate render output path: {path}"));
        }
        let Some(bytes) = files.get(path) else {
            findings.push(format!("render output is missing: {path}"));
            continue;
        };
        let content = String::from_utf8_lossy(bytes);
        let version = profile.get("version").and_then(Value::as_str).unwrap_or("");
        let reviewed = profile
            .get("reviewed_on")
            .and_then(Value::as_str)
            .unwrap_or("");
        if !content.contains(&format!(
            "Profile version: {version} · Reviewed: {reviewed}"
        )) {
            findings.push(format!("render metadata is stale: {path}"));
        }

        let selected_groups: Vec<String> = output
            .get("groups")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect();
        for group in &selected_groups {
            if !group_ids.contains(group) {
                findings.push(format!("render output references an unknown group: {path}"));
            }
        }

        let selected_group_set: HashSet<String> = selected_groups.iter().cloned().collect();
        if selected_group_set.len() != selected_groups.len() {
            findings.push(format!("render output repeats a group: {path}"));
        }
        let canonical_selected_groups: Vec<String> = profile_groups
            .iter()
            .filter_map(|group| group.get("id").and_then(Value::as_str))
            .filter(|group| selected_group_set.contains(*group))
            .map(str::to_owned)
            .collect();
        if selected_groups != canonical_selected_groups {
            findings.push(format!("render groups are out of canonical order: {path}"));
        }

        let Some(mode) = output.get("mode").and_then(Value::as_str) else {
            findings.push(format!("render output has no mode: {path}"));
            continue;
        };

        let selected_ids: Vec<String> = match mode {
            "detailed" => {
                for group in &selected_groups {
                    *detailed_group_counts.entry(group.clone()).or_insert(0) += 1;
                    if group_paths.get(group).map(String::as_str) != Some(path) {
                        findings.push(format!("detailed group path mismatch: {path}"));
                    }
                }
                expected_record_ids_for(&profile_groups, records, &selected_group_set, None)
            }
            "complete_with_contract" => {
                if selected_group_set.len() != group_ids.len()
                    || !group_ids
                        .iter()
                        .all(|group| selected_group_set.contains(group))
                {
                    findings.push(format!("complete export omits a profile group: {path}"));
                }
                complete_paths.push(path.to_owned());
                expected_record_ids_for(&profile_groups, records, &group_ids, None)
            }
            "selected_records" => {
                let ids: Vec<String> = output
                    .get("record_ids")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_str)
                    .map(str::to_owned)
                    .collect();
                let explicit_records: HashSet<String> = ids.iter().cloned().collect();
                if explicit_records.len() != ids.len() {
                    findings.push(format!("compact render selection repeats a record: {path}"));
                }
                for id in &ids {
                    let record = records
                        .as_array()
                        .into_iter()
                        .flatten()
                        .find(|record| record.get("id").and_then(Value::as_str) == Some(id));
                    if record
                        .and_then(|record| record.get("group"))
                        .and_then(Value::as_str)
                        .map(|group| selected_groups.iter().any(|selected| selected == group))
                        != Some(true)
                    {
                        findings.push(format!(
                            "compact record is outside its selected groups: {path}"
                        ));
                    }
                }
                expected_record_ids_for(
                    &profile_groups,
                    records,
                    &selected_group_set,
                    Some(&explicit_records),
                )
            }
            _ => {
                findings.push(format!("unknown render mode: {path}"));
                continue;
            }
        };

        let selected_refs: Vec<&str> = selected_ids.iter().map(String::as_str).collect();
        findings.extend(validate_rendered_records(&content, records, &selected_refs));
    }

    findings.extend(validate_required_render_outputs(
        outputs,
        &group_paths,
        &output_paths,
    ));

    for group in group_ids {
        if detailed_group_counts.get(&group) != Some(&1) {
            findings.push("each profile group must have exactly one detailed output".to_string());
        }
    }

    if complete_paths.len() != 2 {
        findings.push("render specification must define two complete exports".to_string());
    } else if files.get(&complete_paths[0]) != files.get(&complete_paths[1]) {
        findings.push("complete exports are not byte-identical".to_string());
    }

    if let (Some(contract), Some(exports)) = (files.get("AI.md"), complete_paths.first()) {
        if let Some(full_export) = files.get(exports) {
            let contract = normalize_markdown_links(&String::from_utf8_lossy(contract));
            let full_export = normalize_markdown_links(&String::from_utf8_lossy(full_export));
            if !full_export.contains(&contract) {
                findings
                    .push("complete export does not contain the current AI contract".to_string());
            }
        }
    } else {
        findings.push("AI contract or complete export is missing".to_string());
    }

    let budget = spec
        .get("compact_character_budget")
        .and_then(Value::as_u64)
        .unwrap_or(0) as usize;
    match files.get("exports/custom-instructions.txt") {
        Some(bytes) if String::from_utf8_lossy(bytes).chars().count() <= budget => {}
        Some(_) => {
            findings.push("custom instructions exceed the compact character budget".to_string())
        }
        None => findings.push("custom instructions file is missing".to_string()),
    }

    findings
}

fn validate_manifest_metadata(manifest: &Value, profile: &Value) -> Vec<String> {
    let mut findings = Vec::new();
    for key in ["profile_id", "version", "reviewed_on"] {
        if manifest.get(key) != profile.get(key) {
            findings.push(format!("manifest metadata is stale: {key}"));
        }
    }
    if manifest.get("canonical").and_then(Value::as_str) != Some("profile.json") {
        findings.push("manifest canonical file is incorrect".to_string());
    }
    if manifest
        .get("required_paid_dependencies")
        .and_then(Value::as_u64)
        != Some(0)
    {
        findings.push("manifest paid dependency count is not zero".to_string());
    }
    findings
}

pub fn validate_repository(root: &Path) -> Result<ValidationSummary, Vec<String>> {
    let root = root
        .canonicalize()
        .map_err(|_| vec!["repository root is unavailable".to_string()])?;
    let paths = git_tracked_and_unignored_paths(&root).map_err(|error| vec![error])?;
    let mut files = HashMap::new();
    let mut findings = Vec::new();

    for path in &paths {
        if path == "manifest.json" {
            continue;
        }
        let relative = Path::new(path);
        if relative.is_absolute()
            || relative
                .components()
                .any(|component| matches!(component, std::path::Component::ParentDir))
        {
            findings.push("repository contains an unsafe file path".to_string());
            continue;
        }
        match fs::read(root.join(relative)) {
            Ok(bytes) => {
                files.insert(path.clone(), bytes);
            }
            Err(_) => findings.push(format!("tracked or unignored file is unreadable: {path}")),
        }
    }

    let manifest = fs::read(root.join("manifest.json"))
        .map_err(|_| vec!["manifest.json is missing or unreadable".to_string()])?;
    let manifest = parse_json_file(&manifest, "manifest.json").map_err(|error| vec![error])?;
    let profile = files
        .get("profile.json")
        .ok_or_else(|| vec!["profile.json is missing".to_string()])?;
    let profile = parse_json_file(profile, "profile.json").map_err(|error| vec![error])?;
    let schema = files
        .get("schema/profile.schema.json")
        .ok_or_else(|| vec!["profile schema is missing".to_string()])?;
    let schema =
        parse_json_file(schema, "schema/profile.schema.json").map_err(|error| vec![error])?;

    findings.extend(verify_manifest_entries(&manifest, &files));
    findings.extend(validate_profile_structure(&profile));
    findings.extend(validate_manifest_metadata(&manifest, &profile));

    if !schema_uses_only_local_references(&schema) {
        findings.push("profile schema contains an external reference".to_string());
    } else {
        match jsonschema::validator_for(&schema) {
            Ok(validator) => {
                if validator.validate(&profile).is_err() {
                    findings
                        .push("profile.json does not match the committed JSON Schema".to_string());
                }
            }
            Err(_) => findings.push("committed JSON Schema could not be compiled".to_string()),
        }
    }

    let render_spec = files
        .get("render-spec.json")
        .ok_or_else(|| vec!["render-spec.json is missing".to_string()])?;
    let render_spec =
        parse_json_file(render_spec, "render-spec.json").map_err(|error| vec![error])?;
    findings.extend(validate_render_spec(&render_spec, &profile, &files));

    let record_count = profile
        .get("records")
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(0);
    let group_count = profile
        .get("groups")
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(0);
    let source_count = profile
        .get("sources")
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(0);
    if let Some(readme) = files.get("README.md") {
        let readme = String::from_utf8_lossy(readme);
        if !readme.contains(&format!(
            "{record_count} sourced records across {group_count} topics"
        )) {
            findings.push("README profile counts are stale".to_string());
        }
    } else {
        findings.push("README.md is missing".to_string());
    }

    for (path, bytes) in &files {
        if path.ends_with(".md") {
            let content = String::from_utf8_lossy(bytes);
            findings.extend(validate_markdown_links(&root, Path::new(path), &content));
        }
    }

    for finding in scan_git_privacy(&root) {
        findings.push(format!(
            "privacy finding ({}) in {}",
            finding.category, finding.source
        ));
    }

    if findings.is_empty() {
        Ok(ValidationSummary {
            records: record_count,
            groups: group_count,
            sources: source_count,
            files: paths.len(),
        })
    } else {
        findings.sort();
        findings.dedup();
        Err(findings)
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use serde_json::{Value, json};
    use std::{
        collections::{HashMap, HashSet},
        fs,
        path::{Path, PathBuf},
        process::Command,
        sync::atomic::{AtomicU64, Ordering},
    };

    use super::{
        is_valid_gregorian_date, scan_git_privacy, scan_privacy_text,
        schema_uses_only_local_references, validate_markdown_links, validate_profile_structure,
        validate_rendered_records, verify_manifest_entries,
    };

    const SCHEMA: &str = include_str!("../../schema/profile.schema.json");
    const PROFILE: &str = include_str!("../../profile.json");
    static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);

    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            let id = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("profile-validation-{}-{id}", std::process::id()));
            fs::create_dir(&path).expect("create isolated test directory");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

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

    fn init_git(root: &Path) {
        for args in [
            vec!["init", "-q"],
            vec!["config", "user.name", "Validator Test"],
            vec!["config", "user.email", "validator@example.invalid"],
        ] {
            let status = Command::new("git")
                .current_dir(root)
                .args(args)
                .status()
                .expect("git is available for history privacy test");
            assert!(status.success(), "git setup command succeeds");
        }
    }

    fn git(root: &Path, args: &[&str]) {
        let status = Command::new("git")
            .current_dir(root)
            .args(args)
            .status()
            .expect("git is available for history privacy test");
        assert!(status.success(), "git command succeeds");
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

    #[test]
    fn profile_structure_rejects_duplicate_ids_and_dangling_references() {
        let profile = json!({
            "groups": [
                {"id": "core", "path": "PROFILE.md"},
                {"id": "core", "path": "PROFILE.md"}
            ],
            "sources": [{"id": "S01"}, {"id": "S01"}],
            "records": [
                {"id": "rule-one", "group": "missing", "source_ids": ["S99"]},
                {"id": "rule-one", "group": "core", "source_ids": ["S01"]}
            ]
        });
        let findings = validate_profile_structure(&profile);
        assert!(findings.iter().any(|item| item.contains("duplicate group")));
        assert!(
            findings
                .iter()
                .any(|item| item.contains("duplicate source"))
        );
        assert!(
            findings
                .iter()
                .any(|item| item.contains("duplicate record"))
        );
        assert!(findings.iter().any(|item| item.contains("unknown group")));
        assert!(findings.iter().any(|item| item.contains("unknown source")));
    }

    #[test]
    fn manifest_rejects_tampered_hashes_and_unlisted_files() {
        let manifest = json!({
            "files": [{
                "path": "README.md",
                "bytes": 4,
                "sha256": "0000000000000000000000000000000000000000000000000000000000000000"
            }]
        });
        let files = HashMap::from([
            ("README.md".to_string(), b"good".to_vec()),
            ("extra.md".to_string(), b"unexpected".to_vec()),
        ]);
        let findings = verify_manifest_entries(&manifest, &files);
        assert!(
            findings
                .iter()
                .any(|item| item.contains("SHA-256 mismatch"))
        );
        assert!(findings.iter().any(|item| item.contains("unlisted file")));
    }

    #[test]
    fn rendered_exports_reject_missing_duplicate_and_out_of_order_records() {
        let records = json!([
            {"id": "rule-a", "text": "first rule"},
            {"id": "rule-b", "text": "second rule"}
        ]);
        let valid = concat!(
            "<a id=\"record-rule-a\"></a>\n- **rule-a** — first rule\n",
            "<a id=\"record-rule-b\"></a>\n- **rule-b** — second rule\n"
        );
        assert!(validate_rendered_records(valid, &records, &["rule-a", "rule-b"]).is_empty());

        let stale = concat!(
            "<a id=\"record-rule-b\"></a>\n- **rule-b** — second rule\n",
            "<a id=\"record-rule-a\"></a>\n- **rule-a** — first rule\n"
        );
        let findings = validate_rendered_records(stale, &records, &["rule-a", "rule-b"]);
        assert!(!findings.is_empty());

        let missing = "<a id=\"record-rule-a\"></a>\n- **rule-a** — first rule\n";
        assert!(!validate_rendered_records(missing, &records, &["rule-a", "rule-b"]).is_empty());

        let duplicate = concat!(
            "<a id=\"record-rule-a\"></a>\n- **rule-a** — first rule\n",
            "<a id=\"record-rule-a\"></a>\n- **rule-a** — first rule\n"
        );
        assert!(!validate_rendered_records(duplicate, &records, &["rule-a", "rule-b"]).is_empty());
    }

    #[test]
    fn render_spec_requires_all_detailed_full_and_compact_outputs() {
        let spec = json!({
            "outputs": [
                {"path": "PROFILE.md", "mode": "detailed"},
                {"path": "START_HERE.md", "mode": "complete_with_contract"},
                {"path": "exports/agent-instructions.md", "mode": "complete_with_contract"},
                {"path": "exports/compact-profile.md", "mode": "selected_records"}
            ]
        });
        let outputs = spec["outputs"].as_array().unwrap();
        let group_paths = HashMap::from([("core".to_string(), "PROFILE.md".to_string())]);
        let output_paths: HashSet<String> = outputs
            .iter()
            .filter_map(|output| output.get("path").and_then(Value::as_str).map(str::to_owned))
            .collect();
        assert!(
            super::validate_required_render_outputs(outputs, &group_paths, &output_paths).is_empty()
        );

        let missing_compact: Vec<Value> = outputs
            .iter()
            .filter(|output| output.get("mode").and_then(Value::as_str) != Some("selected_records"))
            .cloned()
            .collect();
        let missing_paths: HashSet<String> = missing_compact
            .iter()
            .filter_map(|output| output.get("path").and_then(Value::as_str).map(str::to_owned))
            .collect();
        assert!(
            !super::validate_required_render_outputs(
                &missing_compact,
                &group_paths,
                &missing_paths
            )
            .is_empty()
        );

        let renamed_full: Vec<Value> = outputs
            .iter()
            .map(|output| {
                let mut output = output.clone();
                if output.get("path").and_then(Value::as_str) == Some("START_HERE.md") {
                    output["path"] = Value::String("renamed-full.md".to_string());
                }
                output
            })
            .collect();
        let renamed_paths: HashSet<String> = renamed_full
            .iter()
            .filter_map(|output| output.get("path").and_then(Value::as_str).map(str::to_owned))
            .collect();
        assert!(
            !super::validate_required_render_outputs(
                &renamed_full,
                &group_paths,
                &renamed_paths
            )
            .is_empty()
        );
    }

    #[test]
    fn render_selection_follows_canonical_group_order() {
        let groups = json!([
            {"id": "first"},
            {"id": "second"}
        ]);
        let records = json!([
            {"id": "a-one", "group": "first"},
            {"id": "b-one", "group": "second"},
            {"id": "a-two", "group": "first"}
        ]);
        let selected = HashSet::from(["first".to_string(), "second".to_string()]);
        assert_eq!(
            super::expected_record_ids_for(groups.as_array().unwrap(), &records, &selected, None),
            vec![
                "a-one".to_string(),
                "a-two".to_string(),
                "b-one".to_string()
            ]
        );
    }

    #[test]
    fn markdown_links_reject_missing_files_and_anchors() {
        let root = TempDir::new();
        fs::write(root.path().join("README.md"), "# Existing heading\n").unwrap();
        fs::create_dir(root.path().join("context")).unwrap();
        fs::write(root.path().join("profile.json"), "{}").unwrap();
        fs::write(root.path().join("context/topic.md"), "# Topic\n").unwrap();
        let findings = validate_markdown_links(
            root.path(),
            Path::new("README.md"),
            "[broken](missing.md#not-here)\n[bad anchor](README.md#not-here)\n",
        );
        assert!(findings.len() >= 2);
        let valid_directory_link =
            validate_markdown_links(root.path(), Path::new("README.md"), "[context](context/)\n");
        assert!(valid_directory_link.is_empty());
        let valid_parent_link = validate_markdown_links(
            root.path(),
            Path::new("context/topic.md"),
            "[profile](../profile.json)\n",
        );
        assert!(valid_parent_link.is_empty());
    }

    #[test]
    fn privacy_findings_cover_credentials_and_contact_data_without_echoing_values() {
        let token = format!("ghp_{}", "x".repeat(36));
        let address = ["person", "@", "fixture", ".", "invalid"].concat();
        let phone = ["212", "555", "0100"].join("-");
        let street = ["456", "Example", "Road"].join(" ");
        let card = ["4242", "4242", "4242", "4242"].concat();
        let account = ["1234", "567", "890"].concat();
        let content = format!(
            "token={token}\ncontact {address}\nphone {phone}\naddress: {street}\ncard={card}\naccount number={account}"
        );
        let findings = scan_privacy_text(&content, "fixture.md");
        assert!(findings.iter().any(|item| item.category == "credential"));
        assert!(findings.iter().any(|item| item.category == "email"));
        let rendered = format!("{findings:?}");
        assert!(findings.iter().any(|item| item.category == "phone"));
        assert!(
            findings
                .iter()
                .any(|item| item.category == "street address")
        );
        assert!(findings.iter().any(|item| item.category == "payment card"));
        assert!(
            findings
                .iter()
                .any(|item| item.category == "account number")
        );

        let transcript = [
            "User: synthetic request",
            "Assistant: synthetic answer",
            "User: another synthetic request",
            "",
        ]
        .join("\n");
        assert!(
            scan_privacy_text(&transcript, "transcript-fixture.md")
                .iter()
                .any(|item| item.category == "raw transcript")
        );
        assert!(!rendered.contains(&token));
        assert!(!rendered.contains(&address));
        assert!(!rendered.contains(&phone));
        assert!(!rendered.contains(&street));
        assert!(!rendered.contains(&card));
        assert!(!rendered.contains(&account));
        assert!(
            scan_privacy_text("Public text with [REDACTED] placeholders only.", "safe.md")
                .is_empty()
        );
    }

    #[test]
    fn staged_and_reachable_git_blobs_are_scanned_for_private_material() {
        let root = TempDir::new();
        init_git(root.path());

        let historical_token = format!("ghp_{}", "x".repeat(36));
        fs::write(root.path().join("history.txt"), &historical_token).unwrap();
        git(root.path(), &["add", "history.txt"]);
        git(root.path(), &["commit", "-q", "-m", "synthetic fixture"]);
        fs::write(root.path().join("history.txt"), "safe current content").unwrap();
        git(root.path(), &["add", "history.txt"]);
        git(
            root.path(),
            &["commit", "-q", "-m", "remove synthetic fixture"],
        );

        let staged_email = ["person", "@", "fixture", ".", "invalid"].concat();
        fs::write(root.path().join("staged.txt"), &staged_email).unwrap();
        git(root.path(), &["add", "staged.txt"]);
        fs::write(root.path().join("staged.txt"), "safe working tree").unwrap();

        let findings = scan_git_privacy(root.path());
        assert!(
            findings
                .iter()
                .any(|item| item.source.starts_with("history:"))
        );
        assert!(
            findings
                .iter()
                .any(|item| item.source.starts_with("staged:"))
        );
        let rendered = format!("{findings:?}");
        assert!(!rendered.contains(&historical_token));
        assert!(!rendered.contains(&staged_email));
    }

    #[test]
    fn external_schema_references_are_rejected_before_validation() {
        assert!(schema_uses_only_local_references(&json!({
            "$defs": {"local": {"type": "string"}},
            "$ref": "#/$defs/local"
        })));
        assert!(!schema_uses_only_local_references(&json!({
            "$ref": "https://example.invalid/schema.json"
        })));
        assert!(!schema_uses_only_local_references(&json!({
            "$ref": "../shared/schema.json"
        })));
    }
}
