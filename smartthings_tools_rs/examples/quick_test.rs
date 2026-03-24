use smartthings_tools_rs::SmartThingsToolkit;

fn pp(data: &serde_json::Value) -> String {
    serde_json::to_string_pretty(data).unwrap_or_default()
}

fn main() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    dotenvy::from_path(root.join(".env")).ok();

    let token = std::env::var("SMARTTHINGS_PAT").unwrap_or_default();
    if token.is_empty() || token == "your-personal-access-token-here" {
        eprintln!("SMARTTHINGS_PAT 환경변수가 없습니다. .env 파일을 확인하세요.");
        std::process::exit(1);
    }

    let toolkit = SmartThingsToolkit::new(&token, false);
    println!("등록된 도구: {}개\n", toolkit.list_tool_names().len());

    println!("{}", "=".repeat(60));
    println!(" 1. 디바이스 목록 조회");
    println!("{}", "=".repeat(60));
    let result = toolkit.execute("list_devices", serde_json::json!({}));
    if !result.success {
        eprintln!("실패: {}", result.error.unwrap_or_default());
        std::process::exit(1);
    }

    let devices: Vec<serde_json::Value> = result
        .data
        .as_ref()
        .and_then(|d| d.get("devices"))
        .and_then(|d| d.as_array())
        .cloned()
        .unwrap_or_default();

    println!("발견된 디바이스: {}개\n", devices.len());

    for (i, d) in devices.iter().enumerate() {
        let label = d.get("label").and_then(|l| l.as_str()).unwrap_or("?");
        let did = d.get("device_id").and_then(|v| v.as_str()).unwrap_or("?");
        let caps: Vec<&str> = d
            .get("capabilities")
            .and_then(|c| c.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).take(5).collect())
            .unwrap_or_default();
        let total_caps = d
            .get("capabilities")
            .and_then(|c| c.as_array())
            .map(|a| a.len())
            .unwrap_or(0);
        let extra = if total_caps > 5 { "..." } else { "" };
        println!("  [{i}] {label}");
        println!("      ID: {did}");
        println!("      capabilities: {}{extra}\n", caps.join(", "));
    }

    if devices.is_empty() {
        println!("디바이스가 없습니다. PAT 토큰이 만료되었을 수 있습니다.");
        return;
    }

    let first = &devices[0];
    let first_id = first
        .get("device_id")
        .and_then(|v| v.as_str())
        .unwrap_or("?");
    let first_label = first
        .get("label")
        .and_then(|v| v.as_str())
        .unwrap_or(&first_id[..8.min(first_id.len())]);

    println!("{}", "=".repeat(60));
    println!(" 2. 디바이스 상태 조회: {first_label}");
    println!("{}", "=".repeat(60));
    let result = toolkit.execute(
        "get_device_status",
        serde_json::json!({ "device_id": first_id }),
    );
    if result.success {
        println!(
            "{}",
            pp(result.data.as_ref().unwrap_or(&serde_json::json!(null)))
        );
    } else {
        println!("실패: {}", result.error.unwrap_or_default());
    }

    let sensor = devices.iter().find(|d| {
        d.get("capabilities")
            .and_then(|c| c.as_array())
            .map(|arr| {
                arr.iter().any(|v| {
                    matches!(
                        v.as_str(),
                        Some("temperatureMeasurement" | "relativeHumidityMeasurement")
                    )
                })
            })
            .unwrap_or(false)
    });

    if let Some(s) = sensor {
        let sid = s.get("device_id").and_then(|v| v.as_str()).unwrap_or("?");
        let slabel = s.get("label").and_then(|v| v.as_str()).unwrap_or("?");
        println!("\n{}", "=".repeat(60));
        println!(" 3. 센서 데이터 조회: {slabel}");
        println!("{}", "=".repeat(60));
        let result = toolkit.execute("get_sensor_data", serde_json::json!({ "device_id": sid }));
        if result.success {
            println!(
                "{}",
                pp(result.data.as_ref().unwrap_or(&serde_json::json!(null)))
            );
        } else {
            println!("실패: {}", result.error.unwrap_or_default());
        }
    }

    let plug = devices.iter().find(|d| {
        d.get("capabilities")
            .and_then(|c| c.as_array())
            .map(|arr| arr.iter().any(|v| v.as_str() == Some("powerMeter")))
            .unwrap_or(false)
    });

    if let Some(p) = plug {
        let pid = p.get("device_id").and_then(|v| v.as_str()).unwrap_or("?");
        let plabel = p.get("label").and_then(|v| v.as_str()).unwrap_or("?");
        println!("\n{}", "=".repeat(60));
        println!(" 4. 전력 사용량 조회: {plabel}");
        println!("{}", "=".repeat(60));
        let result = toolkit.execute("get_energy_data", serde_json::json!({ "device_id": pid }));
        if result.success {
            println!(
                "{}",
                pp(result.data.as_ref().unwrap_or(&serde_json::json!(null)))
            );
        } else {
            println!("실패: {}", result.error.unwrap_or_default());
        }
    }

    println!("\n{}", "=".repeat(60));
    println!(" 5. 위치 + 날씨 조회");
    println!("{}", "=".repeat(60));
    let client = smartthings_tools_rs::client::SmartThingsClient::new(&token);
    let loc_data = client.get("/locations", None);
    if let Some(items) = loc_data.get("items").and_then(|i| i.as_array()) {
        if let Some(first) = items.first() {
            let loc_id = first
                .get("locationId")
                .and_then(|v| v.as_str())
                .unwrap_or("?");
            let loc_name = first
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or(&loc_id[..8.min(loc_id.len())]);
            println!("위치: {loc_name}");
            let result =
                toolkit.execute("get_weather", serde_json::json!({ "location_id": loc_id }));
            if result.success {
                println!(
                    "{}",
                    pp(result.data.as_ref().unwrap_or(&serde_json::json!(null)))
                );
            } else {
                println!("실패: {}", result.error.unwrap_or_default());
            }
        }
    } else {
        println!("위치 정보 없음");
    }

    println!("\n샘플 테스트 완료.");
}
