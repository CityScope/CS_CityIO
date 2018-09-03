# cityio server

![](https://user-images.githubusercontent.com/1502180/44991948-cc5bee00-af63-11e8-89ed-e0af00bae0b6.png)


## see [wiki](https://github.com/CityScope/CS_CityIO_backend/wiki) for details.

Backend for the cityscope platform.

- gets data from the tangible interfaces
- serves data to clients to front end and other clients, GAMA, Grasshopper, and Processing. 

## when it dies
[how to recover](https://github.com/CityScope/CS_CityIO_Backend/wiki/how-to-reboot-the-server)

## Data template link

You will find a sample table at ```examples/virtual_table.json``` folder.

[direct link to virtual_table](https://cityio.media.mit.edu/api/table/virtual_table)
[data template](https://github.com/CityScope/CS_CityIO_Backend/wiki/Data-Format)

## run fake table

``` pipenv run python3 examples/fake-table/fake-table.py ```
