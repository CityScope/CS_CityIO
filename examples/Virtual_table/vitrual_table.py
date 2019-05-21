import requests
from datetime import date, timedelta, datetime
import json
import os
import time
import random
import os
import slack
from slack import WebClient

# https://api.slack.com/apps/AJPN6J83W/install-on-team?success=1
# cd ~
# nano .bash_profile
# SLACKBOT = xxxxx


def get_folder_path():
    loc = str(os.path.realpath(
        os.path.join(os.getcwd(), os.path.dirname(__file__)))) + '/'
    return loc


def parse_json_file(field, PATH):
    c = get_folder_path()+PATH
    with open(c) as d:
        data = json.load(d)
    return(data[field])


def demo_grid():
    PATH = 'cityio.json'
    type_arr = []
    cityio_json = parse_json_file('table', PATH)

    for i in range(cityio_json['header']['spatial']['nrows']
                   * cityio_json['header']['spatial']['ncols']):
        this_type = random.randint(0, 4)
        this_rotation = random.randint(0, 3)
        type_arr.append([this_type, this_rotation])
    cityio_json['grid'] = type_arr
    cityio_json = json.dumps(cityio_json)
    return cityio_json


def human_time(this_time):
    human_time = time.strftime(
        '%Y/%m/%d_%H:%M:%S', time.localtime(this_time))
    return human_time


def send_slack_msg(msg):
    # connect to the api and create client
    client = slack.WebClient(token=os.environ['SLACKBOT'])
    # tst slack api
    client.api_call("auth.test")
    # send to slack
    response = client.chat_postMessage(channel='#cityio', text=msg)
    assert response["ok"]
    assert response["message"]["text"] == msg


def loop():
    SEND_INTERVAL = timedelta(milliseconds=1000)
    last_sent = datetime.now()
    API_ENDPOINT = "https://cityio.media.mit.edu/api/table/update/virtual_table"
    # test api error with 'https://httpstat.us/400'
    error_attempts_counter = 0
    first_error_time = None
    cityio_status = 'ok'
    while True:
        from_last_sent = datetime.now() - last_sent
        # if interval passed
        if from_last_sent > SEND_INTERVAL:
            req = requests.post(url=API_ENDPOINT, data=demo_grid())
            # if error in cityIO
            if req.status_code != 200:
                cityio_status = 'dead'
                # mark the first time this error was noted
                if first_error_time == None:
                    first_error_time = human_time(time.time())
                # count the attempts
                error_attempts_counter = error_attempts_counter + 1
                # get longer intervals between atempts
                SEND_INTERVAL = error_attempts_counter * SEND_INTERVAL
                # notify slack
                dead_msg = "cityIO might be down. so sad. it's dead since " + \
                    str(first_error_time) + ' attempt {}, retrying every {} now.'.format(
                        error_attempts_counter, SEND_INTERVAL) + ' :sushi: :sake: ?'
                send_slack_msg(dead_msg)
            else:
                if cityio_status == 'dead':
                    ok_msg = 'https://www.youtube.com/watch?v=TB54dZkzZOY|cityIO \
                    is back to life, back to reality. ' + \
                        str(human_time(time.time()))
                    send_slack_msg(ok_msg)
                    cityio_status = 'ok'
                # reset the counter from prev. counts
                error_attempts_counter = 0
                first_error_time = None
                SEND_INTERVAL = timedelta(milliseconds=1000)
            # reset clock
            last_sent = datetime.now()


if __name__ == '__main__':
    loop()
