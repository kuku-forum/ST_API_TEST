from __future__ import annotations

import json
from typing import Any
from urllib.parse import urlparse

from .models import ApiCallResult, EndpointTestResult, TestSuiteResult

R = "\033[0m"
B = "\033[1m"
D = "\033[2m"
RED = "\033[31m"
GRN = "\033[32m"
YLW = "\033[33m"
BLU = "\033[34m"
CYN = "\033[36m"

H_LINE = "\u2501" * 60
T_LINE = "\u2500" * 56
PIPE = "\u2502"
CORNER_TL = "\u250c"
CORNER_BL = "\u2514"
DASH = "\u2500"
ARROW = "\u21b3"
MDASH = "\u2014"
SKIP = "\u2298"
CHECK = "\u2713"
CROSS = "\u2717"
SEP_86 = "  " + DASH * 86
SEP_52 = "  " + DASH * 52
FOOTER_LINE = DASH * 59


def print_header() -> None:
    print(f"\n{B}{CYN}{H_LINE}{R}")
    print(f"{B}{CYN}  🏠 Samsung SmartThings API 테스트{R}")
    print(f"{B}{CYN}{H_LINE}{R}\n")


def print_category_header(label: str) -> None:
    pad = DASH * max(0, 50 - len(label))
    print(f"\n{B}{BLU}{CORNER_TL}{DASH} {label} {pad}{R}")


def print_category_footer() -> None:
    print(f"{B}{PIPE}{R}")
    print(f"{B}{CORNER_BL}{FOOTER_LINE}{R}")


def print_test_result(result: EndpointTestResult) -> None:
    if result.skipped:
        icon = f"{YLW}{SKIP}{R}"
    elif result.success:
        icon = f"{GRN}{CHECK}{R}"
    else:
        icon = f"{RED}{CROSS}{R}"

    print(f"{B}{PIPE}{R}")
    print(
        f"{B}{PIPE}{R} {icon} {B}{result.endpoint.name}{R} {D}{MDASH} {result.endpoint.description}{R}"
    )

    if result.skipped:
        reason = result.endpoint.needs_setup or "부수효과 테스트 건너뜀"
        print(f"{B}{PIPE}{R}   {YLW}건너뜀: {reason}{R}")
        return

    if result.error:
        print(f"{B}{PIPE}{R}   {RED}오류: {result.error}{R}")

    for call in result.calls:
        _print_api_call(call)


def _print_api_call(call: ApiCallResult) -> None:
    status = call.response["status"]
    sc = GRN if status < 300 else (YLW if status < 400 else RED)

    print(f"{B}{PIPE}{R}   {D}{T_LINE}{R}")
    print(
        f"{B}{PIPE}{R}   {CYN}요청:{R} {B}{call.request['method']}{R} {call.request['url']}"
    )

    if call.request.get("body"):
        print(f"{B}{PIPE}{R}   {CYN}요청 본문:{R}")
        _fmt_json(call.request["body"], 8)

    print(
        f"{B}{PIPE}{R}   {CYN}응답:{R} {sc}{status} {call.response['status_text']}{R} {D}({call.duration}ms){R}"
    )

    body = call.response.get("body")
    if body is not None:
        print(f"{B}{PIPE}{R}   {CYN}응답 데이터:{R}")
        _fmt_json(body, 30)


def _fmt_json(data: Any, max_lines: int) -> None:
    lines = json.dumps(data, indent=2, ensure_ascii=False).split("\n")
    for line in lines[:max_lines]:
        print(f"{B}{PIPE}{R}     {line}")
    if len(lines) > max_lines:
        remaining = len(lines) - max_lines
        print(f"{B}{PIPE}{R}     {D}... ({remaining}줄 생략){R}")


def print_detail_table(suite: TestSuiteResult) -> None:
    print(f"\n{B}{CYN}{H_LINE}{R}")
    print(f"{B}{CYN}  📋 엔드포인트별 상세 결과{R}")
    print(f"{B}{CYN}{H_LINE}{R}\n")

    print(SEP_86)
    hdr = f"  {B}{'#':<4}{'결과':<7}{'카테고리':<16}{'테스트명':<24}{'메서드':<8}{'응답':<8}{'시간':<10}경로{R}"
    print(hdr)
    print(SEP_86)

    for i, r in enumerate(suite.results, 1):
        if r.skipped:
            icon, label = f"{YLW}{SKIP}{R}", "건너뜀"
        elif r.success:
            icon, label = f"{GRN}{CHECK}{R}", "성공"
        else:
            icon, label = f"{RED}{CROSS}{R}", "실패"

        fc = r.calls[0] if r.calls else None
        method = fc.request["method"] if fc else MDASH
        code = str(fc.response["status"]) if fc else MDASH
        dur = f"{fc.duration}ms" if fc else MDASH
        sc = (
            D
            if not fc
            else (
                GRN
                if fc.response["status"] < 300
                else (YLW if fc.response["status"] < 400 else RED)
            )
        )

        path = MDASH
        if fc:
            parsed = urlparse(fc.request["url"])
            q = f"?{parsed.query}" if parsed.query else ""
            path = parsed.path.replace("/v1", "") + q

        cat = r.endpoint.category
        name = r.endpoint.name

        print(
            f"  {D}{i:<4}{R}{icon} {label:<5} {cat:<16}{name:<24}{B}{method:<8}{R}{sc}{code:<8}{R}{D}{dur:<10}{R}{D}{path}{R}"
        )

        extra_label = f"{ARROW} 추가 호출"
        for cc in r.calls[1:]:
            csc = (
                GRN
                if cc.response["status"] < 300
                else (YLW if cc.response["status"] < 400 else RED)
            )
            cp = urlparse(cc.request["url"])
            cq = f"?{cp.query}" if cp.query else ""
            cpath = cp.path.replace("/v1", "") + cq
            cdur = f"{cc.duration}ms"
            print(
                f"  {'':4}{'':7}{'':16}{extra_label:<24}{B}{cc.request['method']:<8}{R}{csc}{cc.response['status']:<8}{R}{D}{cdur:<10}{R}{D}{cpath}{R}"
            )

        if r.error:
            print(f"  {'':4}{'':7}{RED}{ARROW} 오류: {r.error}{R}")

    print(SEP_86)


def print_summary(suite: TestSuiteResult) -> None:
    print(f"\n{B}{CYN}{H_LINE}{R}")
    print(f"{B}{CYN}  📊 테스트 결과 요약{R}")
    print(f"{B}{CYN}{H_LINE}{R}\n")

    dur_sec = suite.duration / 1000
    print(f"  총 소요 시간: {B}{dur_sec:.1f}초{R}")
    print(f"  전체: {B}{suite.total}{R}개")
    print(f"  {GRN}성공: {suite.passed}{R}개")
    print(f"  {RED}실패: {suite.failed}{R}개")
    if suite.skipped > 0:
        print(f"  {YLW}건너뜀: {suite.skipped}{R}개")

    cats: dict[str, dict[str, int]] = {}
    for r in suite.results:
        cat = r.endpoint.category_label
        s = cats.setdefault(cat, {"total": 0, "passed": 0, "failed": 0, "skipped": 0})
        s["total"] += 1
        if r.skipped:
            s["skipped"] += 1
        elif r.success:
            s["passed"] += 1
        else:
            s["failed"] += 1

    print(f"\n  {B}카테고리별 결과:{R}")
    print(SEP_52)
    h_cat = "카테고리"
    h_pass = "성공"
    h_fail = "실패"
    h_skip = "건너뜀"
    print(f"  {B}{h_cat:<24}{h_pass:<8}{h_fail:<8}{h_skip:<8}전체{R}")
    print(SEP_52)
    for cat, s in cats.items():
        pad = cat + " " * max(0, 22 - len(cat))
        print(
            f"  {pad}  {GRN}{s['passed']:<8}{R}{RED}{s['failed']:<8}{R}{YLW}{s['skipped']:<8}{R}{s['total']}"
        )
    print(SEP_52 + "\n")
