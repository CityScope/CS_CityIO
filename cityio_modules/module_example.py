'''example of a simple module'''
import json
import random


class SimpleModule():
    '''simple'''

    def demo_grid(self):
        '''
        Here you define the amazing module
        '''

        data = []
        for _ in range(1, 200):
            a = random.randint(0, 100)
            b = random.randint(0, 100)
            data.append([a, b])

        '''
        Here you save it to local file 
        and that's it! The handler takes care 
        of the rest 
        '''
        with open("results.json", 'w') as outfile:
            json.dump(data, outfile)


if __name__ == "__main__":
    simpleModule = SimpleModule()
    simpleModule.demo_grid()
