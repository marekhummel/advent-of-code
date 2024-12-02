import sqlite3
from glob import glob
from os.path import expanduser
import json

# EDGE_COOKIE_FILE = "~/Appdata/Local/Microsoft/Edge/User Data/Default/Network/Cookies"
EDGE_COOKIE_FILE = "~/Downloads/Cookies"
FIREFOX_COOKIE_FILE = "~/Appdata/Roaming/Mozilla/Firefox/Profiles/*/cookies.sqlite"

def read_edge():
    # Connect to the cookies database
    cookie_file = glob(expanduser(EDGE_COOKIE_FILE))[0]
    conn = sqlite3.connect(cookie_file)
    conn.text_factory = lambda b: b.decode(errors = 'ignore')
    cursor = conn.cursor()

    cursor.execute("PRAGMA table_info(cookies);")
    cookie_table = cursor.fetchall()
    # print([t[1] for t in cookie_table])

    # Query the cookies table
    cursor.execute("""SELECT * FROM cookies WHERE host_key LIKE '%adventofcode%'""")
    cookie_tuples = cursor.fetchall()
    cookies = [{key[1]: value for value, key in  zip(c, cookie_table)} for c in cookie_tuples]

    for c in cookies:
        print(f"'{c["name"]}': '{c["value"]}' | '{c["encrypted_value"]}'")

    # # Close the database connection
    conn.close()

if __name__ == "__main__":
    print(read_edge())