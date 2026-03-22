from __future__ import annotations
from ..models import ApiEndpointTest

from .locations import tests as location_tests
from .rooms import tests as room_tests
from .modes import tests as mode_tests
from .devices import tests as device_tests
from .device_profiles import tests as device_profile_tests
from .capabilities import tests as capability_tests
from .scenes import tests as scene_tests
from .rules import tests as rule_tests
from .apps import tests as app_tests
from .installed_apps import tests as installed_app_tests
from .subscriptions import tests as subscription_tests
from .schedules import tests as schedule_tests
from .schema_connectors import tests as schema_tests
from .services import tests as service_tests
from .history import tests as history_tests

all_tests: list[ApiEndpointTest] = [
    *location_tests,
    *room_tests,
    *mode_tests,
    *device_tests,
    *device_profile_tests,
    *capability_tests,
    *scene_tests,
    *rule_tests,
    *app_tests,
    *installed_app_tests,
    *subscription_tests,
    *schedule_tests,
    *schema_tests,
    *service_tests,
    *history_tests,
]

categories = list(dict.fromkeys(t.category for t in all_tests))


def get_tests_by_category(category: str) -> list[ApiEndpointTest]:
    return [t for t in all_tests if t.category == category]
