#!/usr/bin/env python3
"""
Python/Rust 테스트 로그 병합 및 비교 도구.

사용법:
    python scripts/merge_logs.py                  # latest.json 비교
    python scripts/merge_logs.py --save           # 결과를 scripts/merged/ 에 저장
    python scripts/merge_logs.py --py api_explorer/logs/2026-01-01_00-00-00.json \
                                 --rs api_explorer_rs/logs/2026-01-01_00-00-00.json
"""

from __future__ import annotations

import argparse
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PY_LATEST = ROOT / "api_explorer" / "logs" / "latest.json"
RS_LATEST = ROOT / "api_explorer_rs" / "logs" / "latest.json"
MERGED_DIR = ROOT / "scripts" / "merged"

R = "\033[0m"
B = "\033[1m"
D = "\033[2m"
RED = "\033[31m"
GRN = "\033[32m"
YLW = "\033[33m"
CYN = "\033[36m"

DASH = "\u2500"
LINE = "\u2501" * 60
SEP = "  " + DASH * 100


def load(path: Path) -> dict | None:
    if not path.exists():
        return None
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def status_icon(s: str) -> str:
    if s == "passed":
        return f"{GRN}\u2713{R}"
    if s == "failed":
        return f"{RED}\u2717{R}"
    return f"{YLW}\u2298{R}"


def status_label(s: str) -> str:
    return {"passed": "성공", "failed": "실패", "skipped": "건너뜀"}.get(s, s)


def first_call_ms(result: dict) -> str:
    calls = result.get("calls", [])
    if not calls:
        return "\u2014"
    return f"{calls[0].get('durationMs', 0)}ms"


def build_index(data: dict) -> dict[str, dict]:
    idx: dict[str, dict] = {}
    for r in data.get("results", []):
        key = f"{r['category']}:{r['name']}"
        idx[key] = r
    return idx


def print_comparison(py_data: dict, rs_data: dict) -> dict:
    print(f"\n{B}{CYN}{LINE}{R}")
    print(
        f"{B}{CYN}  \U0001f504 Python vs Rust \ud14c\uc2a4\ud2b8 \uacb0\uacfc \ube44\uad50{R}"
    )
    print(f"{B}{CYN}{LINE}{R}\n")

    py_sum = py_data["summary"]
    rs_sum = rs_data["summary"]

    print(f"  {B}{'':30}{'Python':>12}{'Rust':>12}{'차이':>12}{R}")
    print(f"  {DASH * 66}")
    print(f"  {'전체':<30}{py_sum['total']:>12}{rs_sum['total']:>12}{'':>12}")
    print(f"  {GRN}{'성공':<30}{py_sum['passed']:>12}{rs_sum['passed']:>12}{R}{'':>12}")
    print(f"  {RED}{'실패':<30}{py_sum['failed']:>12}{rs_sum['failed']:>12}{R}{'':>12}")
    print(
        f"  {YLW}{'건너뜀':<30}{py_sum['skipped']:>12}{rs_sum['skipped']:>12}{R}{'':>12}"
    )

    py_ms = py_sum["durationMs"]
    rs_ms = rs_sum["durationMs"]
    diff_ms = rs_ms - py_ms
    diff_str = f"{'+' if diff_ms >= 0 else ''}{diff_ms}ms"
    faster = "Rust" if diff_ms < 0 else "Python"
    print(
        f"  {'소요 시간':<30}{py_ms:>10}ms{rs_ms:>10}ms  {diff_str} ({faster} \ube60\ub984)"
    )

    print(f"\n{SEP}")
    print(
        f"  {B}{'#':<4}{'카테고리':<16}{'테스트명':<24}{'Py결과':<8}{'Py시간':<10}{'Rs결과':<8}{'Rs시간':<10}일치{R}"
    )
    print(SEP)

    py_idx = build_index(py_data)
    rs_idx = build_index(rs_data)
    all_keys = list(dict.fromkeys(list(py_idx.keys()) + list(rs_idx.keys())))

    mismatches = []
    merged_results = []

    for i, key in enumerate(all_keys, 1):
        py_r = py_idx.get(key)
        rs_r = rs_idx.get(key)

        cat = (py_r or rs_r or {}).get("category", "?")
        name = (py_r or rs_r or {}).get("name", "?")

        py_st = py_r["status"] if py_r else "없음"
        rs_st = rs_r["status"] if rs_r else "없음"

        py_ic = status_icon(py_st) if py_r else f"{D}\u2014{R}"
        rs_ic = status_icon(rs_st) if rs_r else f"{D}\u2014{R}"

        py_tm = first_call_ms(py_r) if py_r else "\u2014"
        rs_tm = first_call_ms(rs_r) if rs_r else "\u2014"

        match = py_st == rs_st
        match_str = f"{GRN}\u2713{R}" if match else f"{RED}\u2717{R}"

        if not match:
            mismatches.append((cat, name, py_st, rs_st))

        print(
            f"  {D}{i:<4}{R}{cat:<16}{name:<24}"
            f"{py_ic} {status_label(py_st):<5} {D}{py_tm:<8}{R}"
            f"{rs_ic} {status_label(rs_st):<5} {D}{rs_tm:<8}{R}"
            f"{match_str}"
        )

        merged_results.append(
            {
                "category": cat,
                "name": name,
                "python": {
                    "status": py_st,
                    "durationMs": py_r and int(py_tm.replace("ms", ""))
                    if py_tm != "\u2014"
                    else None,
                },
                "rust": {
                    "status": rs_st,
                    "durationMs": rs_r and int(rs_tm.replace("ms", ""))
                    if rs_tm != "\u2014"
                    else None,
                },
                "match": match,
            }
        )

    print(SEP)

    if mismatches:
        print(
            f"\n  {RED}{B}\u26a0 \uacb0\uacfc \ubd88\uc77c\uce58 ({len(mismatches)}\uac74):{R}"
        )
        for cat, name, pst, rst in mismatches:
            print(f"    {cat}/{name}: Python={pst}, Rust={rst}")
    else:
        print(
            f"\n  {GRN}{B}\u2713 \ubaa8\ub4e0 \ud14c\uc2a4\ud2b8 \uacb0\uacfc \uc77c\uce58{R}"
        )

    print()
    return {
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "python": {"timestamp": py_data.get("timestamp"), "summary": py_sum},
        "rust": {"timestamp": rs_data.get("timestamp"), "summary": rs_sum},
        "comparison": merged_results,
        "allMatch": len(mismatches) == 0,
        "mismatchCount": len(mismatches),
    }


def main() -> None:
    parser = argparse.ArgumentParser(description="Python/Rust 테스트 로그 병합 비교")
    parser.add_argument(
        "--py", type=Path, default=PY_LATEST, help="Python 로그 파일 경로"
    )
    parser.add_argument(
        "--rs", type=Path, default=RS_LATEST, help="Rust 로그 파일 경로"
    )
    parser.add_argument("--save", action="store_true", help="병합 결과를 파일로 저장")
    args = parser.parse_args()

    py_data = load(args.py)
    rs_data = load(args.rs)

    if not py_data:
        print(
            f"  {RED}\u274c Python \ub85c\uadf8\ub97c \ucc3e\uc744 \uc218 \uc5c6\uc2b5\ub2c8\ub2e4: {args.py}{R}",
            file=sys.stderr,
        )
        print(
            f"  \uba3c\uc800 \uc2e4\ud589: uv run python -m api_explorer", file=sys.stderr
        )
        sys.exit(1)

    if not rs_data:
        print(
            f"  {RED}\u274c Rust \ub85c\uadf8\ub97c \ucc3e\uc744 \uc218 \uc5c6\uc2b5\ub2c8\ub2e4: {args.rs}{R}",
            file=sys.stderr,
        )
        print(f"  \uba3c\uc800 \uc2e4\ud589: cd api_explorer_rs && cargo run", file=sys.stderr)
        sys.exit(1)

    merged = print_comparison(py_data, rs_data)

    if args.save:
        MERGED_DIR.mkdir(parents=True, exist_ok=True)
        ts = datetime.now(timezone.utc).strftime("%Y-%m-%d_%H-%M-%S")
        out_path = MERGED_DIR / f"{ts}.json"
        with open(out_path, "w", encoding="utf-8") as f:
            json.dump(merged, f, indent=2, ensure_ascii=False)
        print(f"  \U0001f4be \ubcd1\ud569 \ub85c\uadf8 \uc800\uc7a5: {out_path}\n")


if __name__ == "__main__":
    main()
