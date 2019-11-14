import json

def get():
    with open('data/queue', 'r') as queuefile:
        return json.loads(queuefile.read())

def add(id: int, start: int, end: int):
    with open('data/queue', 'w') as queuefile:
        entries = get()
        entries['entries'].append({
            'id': id,
            'start': start,
            'end': end
        })
        json.dump(entries, queuefile)