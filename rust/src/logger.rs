use serde::Serialize;
use serde_json::json;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::TestSuiteResult;

#[derive(Serialize)]
struct LogSummary {
    total: usize,
    passed: usize,
    failed: usize,
    skipped: usize,
    #[serde(rename = "durationMs")]
    duration_ms: u128,
}

#[derive(Serialize)]
struct LogEntry {
    runner: String,
    timestamp: String,
    summary: LogSummary,
    results: Vec<serde_json::Value>,
}

pub fn save_log(suite: &TestSuiteResult) -> Result<String, String> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let logs_dir = manifest_dir.join("logs");
    if !logs_dir.exists() {
        fs::create_dir_all(&logs_dir).map_err(|e| format!("logs 디렉터리 생성 실패: {e}"))?;
    }

    let now = SystemTime::now();
    let timestamp = iso8601_utc(now);
    let filename = format!("{}.json", file_timestamp(now));

    let results = suite
        .results
        .iter()
        .map(|r| {
            json!({
                "category": r.endpoint.category,
                "categoryLabel": r.endpoint.category_label,
                "name": r.endpoint.name,
                "description": r.endpoint.description,
                "status": if r.skipped { "skipped" } else if r.success { "passed" } else { "failed" },
                "error": r.error,
                "calls": r.calls.iter().map(|c| {
                    json!({
                        "request": c.request,
                        "response": c.response,
                        "durationMs": c.duration_ms
                    })
                }).collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>();

    let entry = LogEntry {
        runner: "rust".to_string(),
        timestamp,
        summary: LogSummary {
            total: suite.total,
            passed: suite.passed,
            failed: suite.failed,
            skipped: suite.skipped,
            duration_ms: suite.duration_ms,
        },
        results,
    };

    let content = serde_json::to_string_pretty(&entry)
        .map_err(|e| format!("로그 JSON 직렬화 실패: {e}"))?;
    let filepath = logs_dir.join(filename);
    let latest = logs_dir.join("latest.json");

    fs::write(&filepath, &content).map_err(|e| format!("로그 파일 저장 실패: {e}"))?;
    fs::write(&latest, &content).map_err(|e| format!("latest.json 저장 실패: {e}"))?;

    Ok(path_to_string(&filepath))
}

fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().to_string()
}

fn iso8601_utc(st: SystemTime) -> String {
    let (year, month, day, hour, minute, second, millis) = split_utc(st);
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        year, month, day, hour, minute, second, millis
    )
}

fn file_timestamp(st: SystemTime) -> String {
    let (year, month, day, hour, minute, second, _) = split_utc(st);
    format!(
        "{:04}-{:02}-{:02}_{:02}-{:02}-{:02}",
        year, month, day, hour, minute, second
    )
}

fn split_utc(st: SystemTime) -> (i64, i64, i64, i64, i64, i64, i64) {
    let duration = st
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    let total_seconds = i64::try_from(duration.as_secs()).unwrap_or(0);
    let millis = i64::from(duration.subsec_millis());

    let days = total_seconds.div_euclid(86_400);
    let sec_of_day = total_seconds.rem_euclid(86_400);

    let (year, month, day) = civil_from_days(days);
    let hour = sec_of_day / 3600;
    let minute = (sec_of_day % 3600) / 60;
    let second = sec_of_day % 60;

    (year, month, day, hour, minute, second, millis)
}

fn civil_from_days(days_since_unix: i64) -> (i64, i64, i64) {
    let z = days_since_unix + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if month <= 2 { 1 } else { 0 };
    (year, month, day)
}
