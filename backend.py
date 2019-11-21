# TODO: this will be completed once the backend is built
# This makes backend act as a passthrough
import squeue
import server
import json

def get_name(id: int) -> str:
    user_data = server.get_user_data(id)
    if user_data:
        return user_data['name']
    else:
        return None

def create_user(id: int, name: str):
    server.create_user(id, name)

def submit(id: int, start: int, end: int):
    squeue.add(id, start, end)

def push():
    server.push_many(squeue.get())