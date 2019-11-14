def add(id: int, start: int, end: int):
    entries = []
    with open('data/queue', 'r') as entryfile:
        entries = json.loads(entryfile.read())
        entries['entries'].append({
            'id': id,
            'start': start,
            'end': end
        })
    with open('data/queue', 'w') as entryfile:
        json.dump(entries, entryfile)