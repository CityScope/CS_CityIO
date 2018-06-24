
class Owner:
    def __init__(self):
        self.name = "Yasushi Sakai"
        self.title = "Research Assistant"
        self.institute = "MIT Media Lab"

    def __iter__(self):
        for k, v in self.__dict__.items():
            yield(k, v)

