from __future__ import annotations

import argparse
import os
import sys
import time

from .client import ApiClient
from .models import (
    ApiEndpointTest,
    DataStore,
    EndpointTestResult,
    TestContext,
    TestSuiteResult,
)
from .api import all_tests, categories, get_tests_by_category
from .reporter import (
    print_header,
    print_category_header,
    print_category_footer,
    print_test_result,
    print_detail_table,
    print_summary,
)
from .logger import save_log

DEP_MAP: dict[str, list[str]] = {
    "installedApp": ["installedAppId"],
    "등록된 디바이스 프로필": ["deviceProfileId"],
    "등록된 규칙": ["ruleId"],
    "등록된 앱": ["appId"],
}


def run_test(endpoint: ApiEndpointTest, ctx: TestContext) -> EndpointTestResult:
    if endpoint.has_side_effect and not ctx.run_side_effects:
        return EndpointTestResult(endpoint, [], True, True)

    if endpoint.needs_setup:
        for keyword, store_keys in DEP_MAP.items():
            if keyword in endpoint.needs_setup:
                if any(not ctx.store.has(k) for k in store_keys):
                    return EndpointTestResult(endpoint, [], True, True)

    try:
        raw = endpoint.test(ctx)
        calls = raw if isinstance(raw, list) else [raw]
        success = all(c.response["status"] < 400 for c in calls)
        return EndpointTestResult(endpoint, calls, success, False)
    except Exception as e:
        return EndpointTestResult(endpoint, [], False, False, error=str(e))


def main() -> None:
    parser = argparse.ArgumentParser(description="Samsung SmartThings API 테스트 도구")
    parser.add_argument("category", nargs="?", help="특정 카테고리만 테스트")
    parser.add_argument(
        "-s", "--side-effects", action="store_true", help="부수효과 테스트 포함"
    )
    parser.add_argument("--no-log", action="store_true", help="로그 파일 저장 안 함")
    args = parser.parse_args()

    token = os.environ.get("SMARTTHINGS_PAT", "")
    if not token or token == "your-personal-access-token-here":
        print(
            "\n❌ SMARTTHINGS_PAT 환경변수가 설정되지 않았습니다.\n"
            "   .env 파일에 PAT 토큰을 설정하세요.\n"
            "   발급: https://account.smartthings.com/tokens\n",
            file=sys.stderr,
        )
        sys.exit(1)

    client = ApiClient(token)
    store = DataStore()
    ctx = TestContext(client=client, store=store, run_side_effects=args.side_effects)

    tests_to_run = get_tests_by_category(args.category) if args.category else all_tests
    if not tests_to_run:
        avail = ", ".join(categories)
        print(
            f"\n❌ 카테고리 '{args.category}'을(를) 찾을 수 없습니다.", file=sys.stderr
        )
        print(f"사용 가능한 카테고리: {avail}", file=sys.stderr)
        sys.exit(1)

    print_header()

    if not args.side_effects:
        print(
            "  ℹ️  부수효과 테스트는 건너뜁니다. 포함하려면 --side-effects 플래그를 사용하세요.\n"
        )

    suite_start = time.perf_counter()
    results: list[EndpointTestResult] = []
    current_category = ""

    for endpoint in tests_to_run:
        if endpoint.category_label != current_category:
            if current_category:
                print_category_footer()
            current_category = endpoint.category_label
            print_category_header(current_category)

        result = run_test(endpoint, ctx)
        results.append(result)
        print_test_result(result)

    if current_category:
        print_category_footer()

    suite_duration = round((time.perf_counter() - suite_start) * 1000)

    suite = TestSuiteResult(
        total=len(results),
        passed=sum(1 for r in results if r.success and not r.skipped),
        failed=sum(1 for r in results if not r.success),
        skipped=sum(1 for r in results if r.skipped),
        duration=suite_duration,
        results=results,
    )

    print_detail_table(suite)
    print_summary(suite)

    if not args.no_log:
        log_path = save_log(suite)
        print(f"  💾 로그 저장: {log_path}\n")

    if suite.failed > 0:
        sys.exit(1)
