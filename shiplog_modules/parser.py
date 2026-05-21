from .models import VoyageRecord


def parse_record(line: str) -> VoyageRecord:
    """Parse a CSV line into a VoyageRecord."""
    vessel, distance, fuel = [part.strip() for part in line.split(",")]
    return VoyageRecord(vessel=vessel, distance_nm=float(distance), fuel_tons=float(fuel))
