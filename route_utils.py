"""Route timing utilities."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class RouteInputs:
    distance_miles: float
    avg_speed_mph: float
    traffic_multiplier: float
    stop_count: int
    stop_minutes: float


def estimate_route_eta_hours(
    distance_miles: float,
    avg_speed_mph: float,
    traffic_multiplier: float,
    stop_count: int,
    stop_minutes: float,
) -> float:
    """Estimate total trip duration in hours."""
    route = RouteInputs(
        distance_miles=distance_miles,
        avg_speed_mph=avg_speed_mph,
        traffic_multiplier=traffic_multiplier,
        stop_count=stop_count,
        stop_minutes=stop_minutes,
    )
    _validate_inputs(route)
    moving_hours = _moving_time_hours(route.distance_miles, route.avg_speed_mph)
    traffic_delay = _traffic_delay_hours(moving_hours, route.traffic_multiplier)
    stop_delay = _stop_delay_hours(route.stop_count, route.stop_minutes)
    return _total_eta_hours(moving_hours, traffic_delay, stop_delay)


def _validate_inputs(route: RouteInputs) -> None:
    if route.distance_miles < 0:
        raise ValueError("distance_miles must be non-negative")
    if route.avg_speed_mph <= 0:
        raise ValueError("avg_speed_mph must be positive")
    if route.traffic_multiplier < 1:
        raise ValueError("traffic_multiplier must be >= 1")
    if route.stop_count < 0:
        raise ValueError("stop_count must be non-negative")
    if route.stop_minutes < 0:
        raise ValueError("stop_minutes must be non-negative")


def _moving_time_hours(distance_miles: float, avg_speed_mph: float) -> float:
    return distance_miles / avg_speed_mph


def _traffic_delay_hours(moving_hours: float, traffic_multiplier: float) -> float:
    return moving_hours * (traffic_multiplier - 1)


def _stop_delay_hours(stop_count: int, stop_minutes: float) -> float:
    return (stop_count * stop_minutes) / 60


def _total_eta_hours(moving_hours: float, traffic_delay: float, stop_delay: float) -> float:
    return moving_hours + traffic_delay + stop_delay
