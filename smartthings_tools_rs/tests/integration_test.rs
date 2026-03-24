use smartthings_tools_rs::tool::ToolResult;
use smartthings_tools_rs::SmartThingsToolkit;

const PASS: &str = "\x1b[92mPASS\x1b[0m";
const FAIL: &str = "\x1b[91mFAIL\x1b[0m";
const SKIP: &str = "\x1b[93mSKIP\x1b[0m";

fn print_result(test_name: &str, result: &ToolResult) {
    let status = if result.success { PASS } else { FAIL };
    println!("  [{status}] {test_name}");
    if result.success {
        if let Some(ref data) = result.data {
            let preview = serde_json::to_string(data).unwrap_or_default();
            let truncated = if preview.len() > 200 {
                &preview[..200]
            } else {
                &preview
            };
            println!("         {truncated}");
        }
    } else if let Some(ref err) = result.error {
        println!("         error: {err}");
    }
}

fn get_token() -> Option<String> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent()?;
    dotenvy::from_path(root.join(".env")).ok();
    let token = std::env::var("SMARTTHINGS_PAT").ok()?;
    if token.is_empty() || token == "your-personal-access-token-here" {
        return None;
    }
    Some(token)
}

#[test]
fn test_tool_registration() {
    let token = match get_token() {
        Some(t) => t,
        None => {
            println!("[{SKIP}] SMARTTHINGS_PAT not set");
            return;
        }
    };

    let toolkit = SmartThingsToolkit::new(&token, true);
    let names = toolkit.list_tool_names();

    println!("\nSmartThings Tool Package (Rust) 테스트");
    println!("등록된 도구: {}개", names.len());
    println!("{}", "=".repeat(50));

    assert_eq!(
        names.len(),
        38,
        "38개 도구가 등록되어야 합니다 (got {})",
        names.len()
    );
    println!("  [{PASS}] {}개 도구 등록 확인", names.len());
}

#[test]
fn test_schema_conversion() {
    let token = match get_token() {
        Some(t) => t,
        None => return,
    };

    let toolkit = SmartThingsToolkit::new(&token, true);

    let openai = toolkit.to_openai_tools();
    assert!(openai
        .iter()
        .all(|s| s.get("type").and_then(|t| t.as_str()) == Some("function")));
    println!("  [{PASS}] OpenAI 형식 변환 ({}개)", openai.len());

    let anthropic = toolkit.to_anthropic_tools();
    assert!(anthropic.iter().all(|s| s.get("input_schema").is_some()));
    println!("  [{PASS}] Anthropic 형식 변환 ({}개)", anthropic.len());
}

#[test]
fn test_api_calls() {
    let token = match get_token() {
        Some(t) => t,
        None => return,
    };

    let toolkit = SmartThingsToolkit::new(&token, true);
    let mut passed = 0u32;
    let mut failed = 0u32;
    let mut skipped = 0u32;

    println!("\n[API 호출 테스트]");

    let result = toolkit.execute("list_devices", serde_json::json!({}));
    print_result("list_devices", &result);
    if result.success {
        passed += 1;
    } else {
        failed += 1;
    }

    let mut device_id: Option<String> = None;
    let mut sensor_id: Option<String> = None;
    let mut plug_id: Option<String> = None;

    if result.success {
        if let Some(ref data) = result.data {
            if let Some(devices) = data.get("devices").and_then(|d| d.as_array()) {
                println!("         발견된 디바이스: {}개", devices.len());
                for d in devices {
                    let caps: Vec<&str> = d
                        .get("capabilities")
                        .and_then(|c| c.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
                        .unwrap_or_default();

                    if device_id.is_none() && caps.contains(&"switch") {
                        device_id = d
                            .get("device_id")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                    }
                    if sensor_id.is_none()
                        && caps.iter().any(|c| {
                            ["temperatureMeasurement", "presenceSensor", "motionSensor"].contains(c)
                        })
                    {
                        sensor_id = d
                            .get("device_id")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                    }
                    if plug_id.is_none() && caps.contains(&"powerMeter") {
                        plug_id = d
                            .get("device_id")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                    }
                }
            }
        }
    }

    if let Some(ref did) = device_id {
        let result = toolkit.execute("get_device_status", serde_json::json!({ "device_id": did }));
        print_result(
            &format!("get_device_status ({}...)", &did[..8.min(did.len())]),
            &result,
        );
        if result.success {
            passed += 1;
        } else {
            failed += 1;
        }
    } else {
        println!("  [{SKIP}] get_device_status (switch 디바이스 없음)");
        skipped += 1;
    }

    if let Some(ref sid) = sensor_id {
        let result = toolkit.execute("get_sensor_data", serde_json::json!({ "device_id": sid }));
        print_result(
            &format!("get_sensor_data ({}...)", &sid[..8.min(sid.len())]),
            &result,
        );
        if result.success {
            passed += 1;
        } else {
            failed += 1;
        }
    } else {
        println!("  [{SKIP}] get_sensor_data (센서 디바이스 없음)");
        skipped += 1;
    }

    if let Some(ref pid) = plug_id {
        let result = toolkit.execute("get_energy_data", serde_json::json!({ "device_id": pid }));
        print_result(
            &format!("get_energy_data ({}...)", &pid[..8.min(pid.len())]),
            &result,
        );
        if result.success {
            passed += 1;
        } else {
            failed += 1;
        }
    } else {
        println!("  [{SKIP}] get_energy_data (플러그 디바이스 없음)");
        skipped += 1;
    }

    let client = smartthings_tools_rs::client::SmartThingsClient::new(&token);
    let loc_data = client.get("/locations", None);
    if let Some(items) = loc_data.get("items").and_then(|i| i.as_array()) {
        if let Some(first) = items.first() {
            if let Some(loc_id) = first.get("locationId").and_then(|v| v.as_str()) {
                let result =
                    toolkit.execute("get_weather", serde_json::json!({ "location_id": loc_id }));
                print_result(
                    &format!("get_weather ({}...)", &loc_id[..8.min(loc_id.len())]),
                    &result,
                );
                if result.success {
                    passed += 1;
                } else {
                    println!("         (날씨 서비스 미지원 위치 — 건너뜀)");
                    skipped += 1;
                }
            }
        }
    }

    println!("\n[입력 검증 테스트]");

    let bad = toolkit.execute(
        "switch_power",
        serde_json::json!({ "device_id": "", "state": "invalid" }),
    );
    if !bad.success {
        println!("  [{PASS}] 잘못된 입력 거부됨");
        passed += 1;
    } else {
        println!("  [{FAIL}] 잘못된 입력이 통과됨");
        failed += 1;
    }

    let unknown = toolkit.execute("nonexistent_tool", serde_json::json!({}));
    if !unknown.success {
        println!(
            "  [{PASS}] 미등록 도구 거부됨: {}",
            unknown.error.unwrap_or_default()
        );
        passed += 1;
    } else {
        println!("  [{FAIL}] 미등록 도구가 통과됨");
        failed += 1;
    }

    let total = passed + failed + skipped;
    println!("\n{}", "=".repeat(50));
    println!("결과: {passed}/{total} 통과, {failed} 실패, {skipped} 건너뜀");

    assert_eq!(failed, 0, "{failed} tests failed");
}
