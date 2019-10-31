#!/usr/bin/env python
import sys
import webbrowser
import json


def ddgo_url(bang):
    with open('ddgo_bangs.json') as f:
        data = json.load(f)
    for entry in data:
        if entry['t'] == bang:
            return entry['u'].replace("{{{s}}}", "{}")
    return None

def get_url(bang):
    bangs = {}
    with open('bangs') as f:
        for line in f:
            (key, val) = line.split()
            bangs[key] = val
    return bangs.get(bang)


def open_browser(bang, query):
    url = get_url(bang)
    if url is None:
        url = ddgo_url(bang)
    if url is not None:
        url = url.replace("{}", query)
        webbrowser.open(url)


if __name__ == "__main__":
    bang = ""
    query = ""
    if len(sys.argv) >= 2:
        bang = sys.argv[1]
    else:
        print("Not enough arguments: Missing bang")
        exit(1)

    if len(sys.argv) >= 3:
        query = " ".join(sys.argv[2:])
    else:
        print("Not enough arguments: Missing query")
        exit(1)

    open_browser(bang, query)
