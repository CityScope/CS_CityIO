import time
import sched
from requests import post
from Table import Table

# base_url = 'https://cityio.media.mit.edu/'
base_url = 'http://localhost:8081/'

table = Table()

url = '{}dev/api/table/update/{}'.format(base_url, table.header.name)

scheduled = sched.scheduler(time.time, time.sleep)

def update(sc=None):
    table.update()
    post(url, json=dict(table))
    print('updated {}'.format(table.header.name))
    if sc is not None:
        scheduled.enter(60, 1, update, (sc,))

update()

scheduled.enter(60, 1, update, (scheduled,))
scheduled.run()

