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
