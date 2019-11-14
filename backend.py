# TODO: this will be completed once the backend is built
# This makes backend act as a passthrough
import dummy_backend
import queue
import json

def get_name(id: int) -> str:
    return dummy_backend.get_name(id)

def create_user(id: int, name: str) -> str:
    return dummy_backend.create_user(id, name)

def submit(id: int, start: int, end: int):
    queue.add(id, start, end)