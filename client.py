import backend
import time
import os

# True if the user is signing in, false if signing out
def sign_in_out(id: int) -> bool:
    # We only store times as ints, but this could be changed easily if more precision is required
    timestamp: int = round(time.mktime(time.localtime()))
    try: # If a sessionfile exists, the user is signed in and must be signed out
        with open('data/sessions/' + str(id), 'r') as sessionfile:
            backend.submit(id, int(sessionfile.read()), timestamp)
        os.remove('data/sessions/' + str(id))
        return False
    except: # Create a new sessionfile
        with open('data/sessions/' + str(id), 'w+') as sessionfile:
            sessionfile.write(str(timestamp))
        return True

while True:
    cmd: str = input('Please scan your ID card or enter your student number: ')
    if cmd == '..':
        # TODO: sign out all users and force the queue to push
        break
    else: # the number should be handled as an ID
        try:
            id: int = int(cmd)
            name = backend.get_name(id)
            if name:
                if sign_in_out(id):
                    print('Welcome, ' + name + '.')
                else:
                    print('Goodbye, ' + name + '.')
            else:
                name = backend.create_user(id, input('This appears to be your first time signing in. Please enter your name: '))
                sign_in_out(id)
                print('Welcome, ' + name + '.')
        except:
            print('Invalid ID or command.')
