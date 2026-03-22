mod api;
mod client;
mod logger;
mod models;
mod reporter;

use clap::Parser;
use std::collections::HashMap;
use std::path::Path;
use std::process;
use std::time::Instant;

use crate::client::ApiClient;
use crate::models::{ApiEndpointTest, EndpointTestResult, TestContext, TestSuiteResult};

#[derive(Parser, Debug)]
#[command(name = "smartthings-rust", about = "Samsung SmartThings API 테스트 도구")]
struct Cli {
    #[arg(help = "특정 카테고리만 테스트")]
    category: Option<String>,

    #[arg(short = 's', long = "side-effects", help = "부수효과 테스트 포함")]
    side_effects: bool,

    #[arg(long = "no-log", help = "로그 파일 저장 안 함")]
    no_log: bool,
}

fn run_test(endpoint: &ApiEndpointTest, ctx: &mut TestContext) -> EndpointTestResult {
    if endpoint.has_side_effect && !ctx.run_side_effects {
        return EndpointTestResult {
            endpoint: endpoint.clone(),
            calls: vec![],
            success: true,
            skipped: true,
            error: None,
        };
    }

    if let Some(needs_setup) = &endpoint.needs_setup {
        let dep_map: Vec<(&str, Vec<&str>)> = vec![
            ("installedApp", vec!["installedAppId"]),
            ("등록된 디바이스 프로필", vec!["deviceProfileId"]),
            ("등록된 규칙", vec!["ruleId"]),
            ("등록된 앱", vec!["appId"]),
        ];

        for (keyword, keys) in dep_map {
            if needs_setup.contains(keyword) && keys.iter().any(|k| !ctx.has_store(k)) {
                return EndpointTestResult {
                    endpoint: endpoint.clone(),
                    calls: vec![],
                    success: true,
                    skipped: true,
                    error: None,
                };
            }
        }
    }

    match (endpoint.test)(ctx) {
        Ok(calls) => {
            let success = calls.iter().all(|c| c.response.status < 400);
            EndpointTestResult {
                endpoint: endpoint.clone(),
                calls,
                success,
                skipped: false,
                error: None,
            }
        }
        Err(err) => EndpointTestResult {
            endpoint: endpoint.clone(),
            calls: vec![],
            success: false,
            skipped: false,
            error: Some(err),
        },
    }
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let root = Path::new(manifest_dir)
        .parent()
        .unwrap_or_else(|| Path::new(manifest_dir));
    dotenvy::from_path(root.join(".env")).ok();

    let args = Cli::parse();

    let token = std::env::var("SMARTTHINGS_PAT").unwrap_or_default();
    if token.is_empty() || token == "your-personal-access-token-here" {
        eprintln!(
            "\n❌ SMARTTHINGS_PAT 환경변수가 설정되지 않았습니다.\n   .env 파일에 PAT 토큰을 설정하세요.\n   발급: https://account.smartthings.com/tokens\n"
        );
        process::exit(1);
    }

    let client = ApiClient::new(token);
    let mut ctx = TestContext {
        client,
        store: HashMap::new(),
        run_side_effects: args.side_effects,
    };

    let all_tests = api::all_tests();
    let tests_to_run = if let Some(category) = &args.category {
        api::get_tests_by_category(&all_tests, category)
    } else {
        all_tests.clone()
    };

    if tests_to_run.is_empty() {
        let categories = api::categories(&all_tests).join(", ");
        if let Some(category) = args.category {
            eprintln!("\n❌ 카테고리 '{category}'을(를) 찾을 수 없습니다.");
            eprintln!("사용 가능한 카테고리: {categories}");
        }
        process::exit(1);
    }

    reporter::print_header();

    if !args.side_effects {
        println!(
            "  ℹ️  부수효과 테스트는 건너뜁니다. 포함하려면 --side-effects 플래그를 사용하세요.\n"
        );
    }

    let started = Instant::now();
    let mut results: Vec<EndpointTestResult> = Vec::new();
    let mut current_category = String::new();

    for endpoint in &tests_to_run {
        if endpoint.category_label != current_category {
            if !current_category.is_empty() {
                reporter::print_category_footer();
            }
            current_category = endpoint.category_label.clone();
            reporter::print_category_header(&current_category);
        }

        let result = run_test(endpoint, &mut ctx);
        reporter::print_test_result(&result);
        results.push(result);
    }

    if !current_category.is_empty() {
        reporter::print_category_footer();
    }

    let duration_ms = started.elapsed().as_millis();
    let suite = TestSuiteResult {
        total: results.len(),
        passed: results.iter().filter(|r| r.success && !r.skipped).count(),
        failed: results.iter().filter(|r| !r.success).count(),
        skipped: results.iter().filter(|r| r.skipped).count(),
        duration_ms,
        results,
    };

    reporter::print_detail_table(&suite);
    reporter::print_summary(&suite);

    if !args.no_log {
        match logger::save_log(&suite) {
            Ok(path) => println!("  💾 로그 저장: {path}\n"),
            Err(err) => eprintln!("  ⚠️ 로그 저장 실패: {err}\n"),
        }
    }

    if suite.failed > 0 {
        process::exit(1);
    }
}
