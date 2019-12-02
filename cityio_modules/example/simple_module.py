'''example of a simple module'''
import json
import random


def demo_grid():
    '''compute something amazing'''

    type_arr = []
    for i in range(1, 200):
        this_type = random.randint(0, 100)
        this_rotation = random.randint(0, 100)
        type_arr.append([this_type, this_rotation])

    cityio_json = json.dumps(type_arr)
    print(cityio_json)
    return cityio_json


if __name__ == '__main__':
    demo_grid()
