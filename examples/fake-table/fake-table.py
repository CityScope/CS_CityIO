import time
import sched
from requests import post, exceptions
from Table import Table

# base_url = 'https://cityio.media.mit.edu/api/'
base_url = 'http://localhost:8080/api/'

table = Table()

url = '{}table/update/{}'.format(base_url, table.header.name)

scheduled = sched.scheduler(time.time, time.sleep)


def update(sc):
    table.makeFakeGrid()
    table.update()
    try:
        res = post(url, json=dict(table))
        print('status code: {}'.format(res.status_code)) 
        print('updated {}'.format(table.header.name))
        print()
    except exceptions.RequestException as e :
        print(e)
        print()

    scheduled.enter(60, 1, update, (sc,))

scheduled.enter(60, 1, update, (scheduled,))
scheduled.run()

