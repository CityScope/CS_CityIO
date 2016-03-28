import socket

BYTE_ORDER = ''

def readbyte(data, signed=True):
    bytes, data = readbytes(data, 1)
    byte = int.from_bytes(bytes, byteorder='little', signed=signed)
    return byte, data


def readbytes(data, len):
    return data[:len], data[len:]

def int_to_bytes(num, len=4, signed=True):
    return num.to_bytes(len, byteorder="little", signed=signed)

def readint(data,len=4, signed=True):
    bytes, data = readbytes(data, len)
    byte = int.from_bytes(bytes, byteorder='little', signed=signed)
    return byte, data

def readshort(data, signed=True):
    return readint(data, 2, signed)

def readstring(data) -> (str, bytes):
    length, data = readbyte(data)
    string, data = readbytes(data, length)
    return str(string), data

def readobject(data) -> (int, bytes, bytes):
    id, data_new = readint(data, 2)
    len, _ = readint(data_new, 2)
    obj, data = readbytes(data, len+4) #read id and len as well
    return id, obj, data

def log(msg, data) -> None:
    """ type: (bytes) -> None
    """
    # print(data, file="log.txt")
    pass