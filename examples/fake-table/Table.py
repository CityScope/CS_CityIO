import json
import time
import random

class Table:

    rotations = [0, 90, 180, 270]
    

    def __init__(self, rows = 20, cols = 20) :
        self.header = dict()
        self.meta = dict()
        self.grid = []
        self.objects = dict()

        self.header['spatial'] = {
                'name' : 'fake_table',
                'row' : rows,
                'col' : cols,
                'latitude' : 42.360357,
                'longitude': -71.087264,
                'rotation' : 0.1,
                'cellSize' : 10
                }

        self.header['block'] = ['rotation', 'height', 'type']

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

        # self.grid = [[0, 0, 90, 0, 30.0]]
        self.grid = []

    def update(self):
        self.meta['timestamp'] = int(time.time())

    def makeFakeGrid(self):
        self.grid = []
        for x in range(self.header['spatial']['row']):
            for y in range(self.header['spatial']['col']):
                fakeCell = []
                fakeCell.append(Table.rotations[random.randrange(len(Table.rotations ) - 1)])
                fakeCell.append(random.randrange(10))
                fakeCell.append(random.randrange(len(self.header['mapping']['type'])))
                
                # self.grid[x + y * self.header['spatial']['row']] = fakeCell
                self.grid.append(fakeCell)


    def toDict(self):
        return {
                'header': self.header,
                'meta' : self.meta,
                'grid' : self.grid,
                'objects' : self.objects 
                }

