import json

def get():
    with open('data/queue', 'r') as queuefile:
        return json.load(queuefile)

def add(id: int, start: int, end: int):
    entries = get()
    with open('data/queue', 'w') as queuefile:
        entries.append({
            'id': id,
            'start': start,
            'end': end
        })
        json.dump(entries, queuefile)
