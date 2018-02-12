from random import choice, randrange
from Spatial import Spatial
from Mapping import Mapping


class Header:

    rotations = [0, 90, 180, 270]
    block = ['type', 'height', 'rotation']

    def __init__(self, name='virtual_table'):
        self.name = name
        self.spatial = Spatial()
        self.block = Header.block
        self.mapping = Mapping()

    def __iter__(self):
        yield('name', self.name)
        yield('spatial', dict(self.spatial))
        yield('block', self.block)
        yield('mapping', dict(self.mapping))

    def totalCellNum(self):
        return self.spatial.nrows * self.spatial.ncols

    def makeFakeCell(self):
        cell = []

        cell.append(self.mapping.randtype())
        cell.append(randrange(10))  # height
        cell.append(choice(Header.rotations))

        return cell


