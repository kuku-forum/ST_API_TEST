from __future__ import annotations

import json
import shutil
from datetime import datetime, timezone
from pathlib import Path

from .models import TestSuiteResult

LOGS_DIR = Path(__file__).resolve().parent.parent / "logs"


def save_log(suite: TestSuiteResult) -> str:
    LOGS_DIR.mkdir(parents=True, exist_ok=True)

    now = datetime.now(timezone.utc)
    entry = {
        "runner": "python",
        "timestamp": now.isoformat(),
        "summary": {
            "total": suite.total,
            "passed": suite.passed,
            "failed": suite.failed,
            "skipped": suite.skipped,
            "durationMs": suite.duration,
        },
        "results": [
            {
                "category": r.endpoint.category,
                "categoryLabel": r.endpoint.category_label,
                "name": r.endpoint.name,
                "description": r.endpoint.description,
                "status": "skipped"
                if r.skipped
                else ("passed" if r.success else "failed"),
                "error": r.error,
                "calls": [
                    {
                        "request": c.request,
                        "response": c.response,
                        "durationMs": c.duration,
                    }
                    for c in r.calls
                ],
            }
            for r in suite.results
        ],
    }

    filename = now.strftime("%Y-%m-%d_%H-%M-%S") + ".json"
    filepath = LOGS_DIR / filename
    latest = LOGS_DIR / "latest.json"

    filepath.write_text(
        json.dumps(entry, indent=2, ensure_ascii=False), encoding="utf-8"
    )
    shutil.copy2(filepath, latest)
    return str(filepath)
