"""Copy the Advent of Code session cookie from a Firefox profile."""

import sqlite3
import sys
from contextlib import closing
from pathlib import Path

FIREFOX_COOKIE_FILES = (
    ".mozilla/firefox/*/cookies.sqlite",
    ".config/mozilla/firefox/*/cookies.sqlite",
    ".var/app/org.mozilla.firefox/.mozilla/firefox/*/cookies.sqlite",
    ".var/app/org.mozilla.firefox/config/mozilla/firefox/*/cookies.sqlite",
    "AppData/Roaming/Mozilla/Firefox/Profiles/*/cookies.sqlite",
)
COOKIE_FILE = Path(__file__).with_name("aoc_cookie.txt")


def read_firefox() -> int:
    cookie_files = [
        path for pattern in FIREFOX_COOKIE_FILES for path in sorted(Path.home().glob(pattern))
    ]
    if not cookie_files:
        print(
            "No Firefox cookie database found. Open Firefox and log in to Advent of Code.",
            file=sys.stderr,
        )
        return 1

    for cookie_file in cookie_files:
        with closing(sqlite3.connect(f"{cookie_file.as_uri()}?mode=ro", uri=True)) as connection:
            cookie = connection.execute(
                "SELECT value FROM moz_cookies "
                "WHERE host IN ('adventofcode.com', '.adventofcode.com') "
                "AND name = 'session' AND value != '' "
                "ORDER BY lastAccessed DESC LIMIT 1"
            ).fetchone()
        if cookie:
            COOKIE_FILE.write_text(cookie[0] + "\n")
            print("Cookie updated")
            return 0

    print(
        "No Advent of Code session cookie found. Log in to adventofcode.com in Firefox.",
        file=sys.stderr,
    )
    return 1


if __name__ == "__main__":
    try:
        sys.exit(read_firefox())
    except (sqlite3.Error, OSError) as error:
        sys.exit(f"Could not import the Firefox cookie: {error}")
