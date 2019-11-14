# This file follows the structure of backend.py and thus should be a drop-in replacement

def get_name(id: int) -> str:
    print(id)
    try:
        with open('data/users/' + str(id), 'r') as userfile:
            return userfile.read()
    except:
        return None

def create_user(id: int, name: str) -> str:
    with open('data/users/' + str(id), 'w+') as userfile:
        userfile.write(name)
        return name

def submit(id: int, start: int, end: int):
    print(str(id) + ' / ' + str(start) + '-' + str(end))