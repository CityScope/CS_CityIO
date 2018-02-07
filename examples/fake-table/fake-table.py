import time
import sched
from requests import post
from Table import Table

base_url = 'https://cityio.media.mit.edu/api/'

table = Table()

url = '{}table/update/{}'.format(base_url, table.header.name)

scheduled = sched.scheduler(time.time, time.sleep)


def update(sc):
    table.makeFakeGrid()
    table.update()
    post(url, json=dict(table))
    print('updated {}'.format(table.header.name))
    scheduled.enter(60, 1, update, (sc,))


scheduled.enter(60, 1, update, (scheduled,))
scheduled.run()

