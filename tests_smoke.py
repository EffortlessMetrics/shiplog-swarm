from shiplog_modules import summarize_efficiency


def test_summarize_efficiency() -> None:
    rows = [
        "Aurora,100,10",
        "Aurora,120,12",
        "Borealis,80,8",
    ]
    assert summarize_efficiency(rows) == {"Aurora": 10.0, "Borealis": 10.0}


if __name__ == "__main__":
    test_summarize_efficiency()
    print("ok")
