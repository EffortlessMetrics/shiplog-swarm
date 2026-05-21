from shiplog import summarize_ship_logs


def test_summarize_ship_logs_breakdown_pipeline():
    data = [
        " Aurora , ACTIVE , 2 ",
        "Aurora,idle,1",
        "Borealis,active,3",
        "invalid",
    ]

    assert summarize_ship_logs(data) == {
        "AURORA": {"total_duration": 3.0, "active_ratio": 2 / 3, "event_count": 2.0},
        "BOREALIS": {"total_duration": 3.0, "active_ratio": 1.0, "event_count": 1.0},
    }
