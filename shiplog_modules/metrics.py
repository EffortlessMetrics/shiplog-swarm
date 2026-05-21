from .models import VoyageRecord


def calculate_efficiency(record: VoyageRecord) -> float:
    """Calculate nautical miles per ton of fuel."""
    if record.fuel_tons <= 0:
        raise ValueError("fuel_tons must be positive")
    return record.distance_nm / record.fuel_tons
