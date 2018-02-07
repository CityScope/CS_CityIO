from Header import Header
from Meta import Meta


class Table:

    rotations = [0, 90, 180, 270]

    def __init__(self):
        self.header = Header()
        self.meta = Meta()
        self.grid = []
        self.objects = dict()

    def __iter__(self):
        yield('header', dict(self.header))
        yield('meta', dict(self.meta))
        yield('objects', self.objects)
        yield('grid', self.grid)

    def update(self):
        self.meta.update()

    def makeFakeGrid(self):
        # TODO: should I prefix the grid length??
        self.grid = []
        for i in range(self.header.totalCellNum()):
            self.grid.append(self.header.makeFakeCell())

