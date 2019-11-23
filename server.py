# This should be the only file to interact directly with the server

import requests

with open('data/api_key', 'r') as keyfile:
    API_KEY = str.strip(keyfile.read())

def req(endpoint: str, data, server: str = 'https://team6479-signin.herokuapp.com'):
    return requests.post(url = server + endpoint, data = data)

def push(id: int, start: int, end: int):
    req('/api/put/entry', {
        'id': id,
        'start': start,
        'end': end,
        'key': API_KEY
    })

def push_many(entries):
    for entry in entries:
        push(entry['id'], entry['start'], entry['end'])

def get_user_data(id: int):
    resp = req('/api/get/user', {'id': id, 'key': API_KEY})
    if resp.status_code == 404:
        return None
    else:
        return resp.json()

def create_user(id: int, name: str):
    req('/api/put/user', {'id': id, 'name': name, 'key': API_KEY})