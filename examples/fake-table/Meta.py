from time import time
from random import choice, random


class Meta:
    def __init__(self):
        self.apiv = '2.0.0'
        self.timestamp = int(time() * 1000)
        self.id = Meta.makeId()

    def __iter__(self):
        for k, v in self.__dict__.items():
            yield(k, v)

    def update(self):
        self.timestamp = int(time() * 1000)

    def makeId(digits=7):
        newId = ''
        alphabet = 'abcdefghijklmnopqrstuvwxyz'

        for i in range(digits):
            sign = choice(alphabet)
            sign = sign if random() < 0.5 else sign.upper()
            newId += sign

        return newId


