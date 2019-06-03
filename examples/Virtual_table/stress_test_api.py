import requests
from datetime import date, timedelta, datetime
import json
import os
import time
import random
import os
import sys
import socket
# pip install RandomWords
from random_word import RandomWords

r = RandomWords()
BASE_URL = "https://cityio.media.mit.edu"


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


def loop():
    SEND_INTERVAL = timedelta(milliseconds=100)
    last_sent = datetime.now()
    API_ENDPOINT = BASE_URL + "/api/table/update/stress_test_new_api/"
    words = r.get_random_words(hasDictionaryDef="true")
    # test api error with 'https://httpstat.us/400'

    while True:
        from_last_sent = datetime.now() - last_sent
        # if interval passed
        if from_last_sent > SEND_INTERVAL:
            print('t')
            for word in words:
                word = " " + word + str(random.random())
                print(word)
                requests.post(url=API_ENDPOINT + word +
                              '/', data=demo_grid())
            # reset clock
            last_sent = datetime.now()


if __name__ == '__main__':
    loop()
