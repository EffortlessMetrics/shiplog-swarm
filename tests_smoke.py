from src.shiplog import process_shipment_manifest


def test_process_shipment_manifest_smoke():
    report = process_shipment_manifest([
        {"tracking_id": " ab-1 ", "destination": "NY", "weight_kg": "1.5", "priority": "EXPRESS"},
        {"tracking_id": "", "destination": "LA", "weight_kg": 3},
    ])

    assert report["metrics"]["received"] == 2
    assert report["metrics"]["accepted"] == 1
    assert report["metrics"]["rejected"] == 1
