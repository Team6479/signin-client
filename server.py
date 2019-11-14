# This should be the only file to interact directly with the server

def push(id: int, start: int, end: int):
    # TODO: push to server
    print('Use the dummy server for now')

def push_many(entries):
    for entry in entries:
        push(entry['id'], entry['start'], entry['end'])
