import time, sched, requests
from Table import Table

base_url = 'https://cityio.media.mit.edu/api/'
table = Table()
url = '{}table/update/{}'.format(base_url, table.header['spatial']['name'])

scheduled  = sched.scheduler(time.time, time.sleep)

def update(sc):
    table.update()
    requests.post(url, json = table.toDict())
    print('updated table')
    scheduled.enter(60, 1, update, (sc,))

scheduled.enter(60, 1, update, (scheduled,))

scheduled.run()



