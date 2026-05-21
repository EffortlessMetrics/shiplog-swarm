from dataclasses import dataclass


@dataclass(frozen=True)
class VoyageRecord:
    vessel: str
    distance_nm: float
    fuel_tons: float
