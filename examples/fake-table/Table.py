import json
import time

class Table:
    

    def __init__(self) :
        self.header = dict()
        self.meta = dict()
        self.grid = []
        self.objects = dict()

        self.header['spatial'] = {
                'name' : 'fake_table',
                'row' : 1,
                'col' : 1,
                'latitude' : 42.360357,
                'longitude': -71.087264,
                'rotation' : 0.1,
                'cellSize' : 10
                }

        self.header['block'] = ['x', 'y', 'rotation', 'type', 'height']

        self.header['mapping'] = dict()
        self.header['mapping']['type'] = {
                '0' : 'RL',
                '1' : 'RM',
                '2' : 'RS',
                '3' : 'OL',
                '4' : 'OM',
                '5' : 'OS',
                '6' : 'ROAD',
                }
        
        self.meta = {
                'apiv' : 2,
                'timestamp' : 1517076690,
                'id' : 'eWRhpRV'
                }

        self.grid = [[0, 0, 90, 0, 30.0]]

    def update(self):
        self.meta['timestamp'] = int(time.time())


    def toDict(self):
        return {
                'header': self.header,
                'meta' : self.meta,
                'grid' : self.grid,
                'objects' : self.objects 
                }

