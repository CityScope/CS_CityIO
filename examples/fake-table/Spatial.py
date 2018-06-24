class Spatial:
    def __init__(self,
                 nrows=20,
                 ncols=20,
                 longitude=-71.087264,
                 physical_longitude=-71.087264,
                 latitude=42.360357,
                 physical_latitude=42.360357,
                 rotation=0.0,
                 cellsize=10
                 ):
        self.ncols = ncols
        self.nrows = nrows
        # longitude = longitude
        self.longitude = longitude
        self.physical_longitude = physical_longitude
        self.latitude = latitude
        self.physical_latitude = physical_latitude
        self.rotation = rotation
        self.cellsize = cellsize

    def __iter__(self):
        for k, v in self.__dict__.items():
            yield(k, v)

