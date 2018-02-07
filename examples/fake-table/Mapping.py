from random import choice


class Mapping:

    type = {
            '0': 'RL',
            '1': 'RM',
            '2': 'RS',
            '3': 'OL',
            '4': 'OM',
            '5': 'OS',
            '6': 'ROAD',
            }

    def __init__(self):
        self.type = Mapping.type

    def __iter__(self):
        for k, v in self.__dict__.items():
            yield(k, v)

    def randtype(self):
        return int(choice(list(self.type)))

