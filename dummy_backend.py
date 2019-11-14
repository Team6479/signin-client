# This file follows the structure of backend.py and thus should be a drop-in replacement

import json
import time

def get_name(id: int) -> str:
    try:
        with open('data/users/' + str(id), 'r') as userfile:
            return userfile.read()
    except:
        return None

def create_user(id: int, name: str) -> str:
    with open('data/users/' + str(id), 'x') as userfile:
        userfile.write(name)
        return name

def submit(id: int, start: int, end: int):
    print('here')
    entries = {}
    with open('data/entries', 'r') as entryfile:
        entries = json.loads(entryfile.read())
        print(entries)
        entries['entries'].append({
            'date': time.strftime('%Y-%m-%d', time.localtime(start)),
            'id': id,
            'start': start,
            'end': end,
            'duration': (end - start)
        })
    with open('data/entries', 'w') as entryfile:
        json.dump(entries, entryfile)