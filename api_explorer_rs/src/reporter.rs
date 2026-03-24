use serde_json::Value;

use crate::models::{ApiCallResult, EndpointTestResult, TestSuiteResult};

const R: &str = "\x1b[0m";
const B: &str = "\x1b[1m";
const D: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const GRN: &str = "\x1b[32m";
const YLW: &str = "\x1b[33m";
const BLU: &str = "\x1b[34m";
const CYN: &str = "\x1b[36m";

const H_LINE: &str = "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━";
const T_LINE: &str = "────────────────────────────────────────────────────────";
const PIPE: &str = "│";
const CORNER_TL: &str = "┌";
const CORNER_BL: &str = "└";
const DASH: &str = "─";
const ARROW: &str = "↳";
const MDASH: &str = "—";
const SKIP: &str = "⊘";
const CHECK: &str = "✓";
const CROSS: &str = "✗";

pub fn print_header() {
    println!("\n{B}{CYN}{H_LINE}{R}");
    println!("{B}{CYN}  🏠 Samsung SmartThings API 테스트{R}");
    println!("{B}{CYN}{H_LINE}{R}\n");
}

pub fn print_category_header(label: &str) {
    let pad_len = 50usize.saturating_sub(label.chars().count());
    let pad = DASH.repeat(pad_len);
    println!("\n{B}{BLU}{CORNER_TL}{DASH} {label} {pad}{R}");
}

pub fn print_category_footer() {
    println!("{B}{PIPE}{R}");
    println!("{B}{CORNER_BL}{}{R}", DASH.repeat(59));
}

pub fn print_test_result(result: &EndpointTestResult) {
    let icon = if result.skipped {
        format!("{YLW}{SKIP}{R}")
    } else if result.success {
        format!("{GRN}{CHECK}{R}")
    } else {
        format!("{RED}{CROSS}{R}")
    };

    println!("{B}{PIPE}{R}");
    println!(
        "{B}{PIPE}{R} {icon} {B}{}{R} {D}{MDASH} {}{R}",
        result.endpoint.name, result.endpoint.description
    );

    if result.skipped {
        let reason = result
            .endpoint
            .needs_setup
            .clone()
            .unwrap_or_else(|| "부수효과 테스트 건너뜀".to_string());
        println!("{B}{PIPE}{R}   {YLW}건너뜀: {reason}{R}");
        return;
    }

    if let Some(error) = &result.error {
        println!("{B}{PIPE}{R}   {RED}오류: {error}{R}");
    }

    for call in &result.calls {
        print_api_call(call);
    }
}

fn print_api_call(call: &ApiCallResult) {
    let status = call.response.status;
    let sc = if status < 300 {
        GRN
    } else if status < 400 {
        YLW
    } else {
        RED
    };

    println!("{B}{PIPE}{R}   {D}{T_LINE}{R}");
    println!(
        "{B}{PIPE}{R}   {CYN}요청:{R} {B}{}{R} {}",
        call.request.method, call.request.url
    );

    if let Some(body) = &call.request.body {
        println!("{B}{PIPE}{R}   {CYN}요청 본문:{R}");
        fmt_json(body, 8);
    }

    println!(
        "{B}{PIPE}{R}   {CYN}응답:{R} {sc}{} {}{R} {D}({}ms){R}",
        call.response.status, call.response.status_text, call.duration_ms
    );

    if let Some(body) = &call.response.body {
        println!("{B}{PIPE}{R}   {CYN}응답 데이터:{R}");
        fmt_json(body, 30);
    }
}

fn fmt_json(data: &Value, max_lines: usize) {
    let rendered = serde_json::to_string_pretty(data).unwrap_or_else(|_| "null".to_string());
    let lines: Vec<&str> = rendered.lines().collect();

    for line in lines.iter().take(max_lines) {
        println!("{B}{PIPE}{R}     {line}");
    }

    if lines.len() > max_lines {
        let remaining = lines.len() - max_lines;
        println!("{B}{PIPE}{R}     {D}... ({remaining}줄 생략){R}");
    }
}

pub fn print_detail_table(suite: &TestSuiteResult) {
    println!("\n{B}{CYN}{H_LINE}{R}");
    println!("{B}{CYN}  📋 엔드포인트별 상세 결과{R}");
    println!("{B}{CYN}{H_LINE}{R}\n");

    let sep_86 = format!("  {}", DASH.repeat(86));
    println!("{sep_86}");
    println!(
        "  {B}{:<4}{:<7}{:<16}{:<24}{:<8}{:<8}{:<10}경로{R}",
        "#", "결과", "카테고리", "테스트명", "메서드", "응답", "시간"
    );
    println!("{sep_86}");

    for (i, r) in suite.results.iter().enumerate() {
        let (icon, label) = if r.skipped {
            (format!("{YLW}{SKIP}{R}"), "건너뜀")
        } else if r.success {
            (format!("{GRN}{CHECK}{R}"), "성공")
        } else {
            (format!("{RED}{CROSS}{R}"), "실패")
        };

        let first_call = r.calls.first();
        let method = first_call
            .map(|c| c.request.method.clone())
            .unwrap_or_else(|| MDASH.to_string());
        let code = first_call
            .map(|c| c.response.status.to_string())
            .unwrap_or_else(|| MDASH.to_string());
        let dur = first_call
            .map(|c| format!("{}ms", c.duration_ms))
            .unwrap_or_else(|| MDASH.to_string());

        let sc = if let Some(fc) = first_call {
            if fc.response.status < 300 {
                GRN
            } else if fc.response.status < 400 {
                YLW
            } else {
                RED
            }
        } else {
            D
        };

        let path = first_call
            .map(|c| extract_path(&c.request.url))
            .unwrap_or_else(|| MDASH.to_string());

        println!(
            "  {D}{:<4}{R}{} {:<5} {:<16}{:<24}{B}{:<8}{R}{}{:<8}{R}{D}{:<10}{R}{D}{}{R}",
            i + 1,
            icon,
            label,
            r.endpoint.category,
            r.endpoint.name,
            method,
            sc,
            code,
            dur,
            path
        );

        for call in r.calls.iter().skip(1) {
            let csc = if call.response.status < 300 {
                GRN
            } else if call.response.status < 400 {
                YLW
            } else {
                RED
            };

            println!(
                "  {:4}{:7}{:16}{:<24}{B}{:<8}{R}{}{:<8}{R}{D}{:<10}{R}{D}{}{R}",
                "",
                "",
                "",
                format!("{ARROW} 추가 호출"),
                call.request.method,
                csc,
                call.response.status,
                format!("{}ms", call.duration_ms),
                extract_path(&call.request.url)
            );
        }

        if let Some(err) = &r.error {
            println!("  {:4}{:7}{RED}{ARROW} 오류: {err}{R}", "", "");
        }
    }

    println!("{sep_86}");
}

pub fn print_summary(suite: &TestSuiteResult) {
    println!("\n{B}{CYN}{H_LINE}{R}");
    println!("{B}{CYN}  📊 테스트 결과 요약{R}");
    println!("{B}{CYN}{H_LINE}{R}\n");

    let duration_sec = suite.duration_ms as f64 / 1000.0;
    println!("  총 소요 시간: {B}{duration_sec:.1}초{R}");
    println!("  전체: {B}{}{R}개", suite.total);
    println!("  {GRN}성공: {}{R}개", suite.passed);
    println!("  {RED}실패: {}{R}개", suite.failed);
    if suite.skipped > 0 {
        println!("  {YLW}건너뜀: {}{R}개", suite.skipped);
    }

    let mut cats: Vec<(String, (usize, usize, usize, usize))> = Vec::new();
    for r in &suite.results {
        let label = r.endpoint.category_label.clone();
        if let Some((_, counts)) = cats.iter_mut().find(|(k, _)| *k == label) {
            counts.0 += 1;
            if r.skipped {
                counts.3 += 1;
            } else if r.success {
                counts.1 += 1;
            } else {
                counts.2 += 1;
            }
        } else {
            let mut counts = (1usize, 0usize, 0usize, 0usize);
            if r.skipped {
                counts.3 = 1;
            } else if r.success {
                counts.1 = 1;
            } else {
                counts.2 = 1;
            }
            cats.push((label, counts));
        }
    }

    println!("\n  {B}카테고리별 결과:{R}");
    let sep_52 = format!("  {}", DASH.repeat(52));
    println!("{sep_52}");
    println!(
        "  {B}{:<24}{:<8}{:<8}{:<8}전체{R}",
        "카테고리", "성공", "실패", "건너뜀"
    );
    println!("{sep_52}");
    for (cat, (total, passed, failed, skipped)) in cats {
        let pad = format!("{}{}", cat, " ".repeat(22usize.saturating_sub(cat.chars().count())));
        println!(
            "  {}  {GRN}{:<8}{R}{RED}{:<8}{R}{YLW}{:<8}{R}{}",
            pad, passed, failed, skipped, total
        );
    }
    println!("{}\n", sep_52);
}

fn extract_path(url: &str) -> String {
    let path_with_query = if let Some(scheme_pos) = url.find("://") {
        let rest = &url[(scheme_pos + 3)..];
        if let Some(path_idx) = rest.find('/') {
            rest[path_idx..].to_string()
        } else {
            "/".to_string()
        }
    } else {
        url.to_string()
    };

    if let Some(stripped) = path_with_query.strip_prefix("/v1") {
        stripped.to_string()
    } else {
        path_with_query
    }
}
