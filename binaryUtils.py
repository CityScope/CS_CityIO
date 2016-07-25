import struct
import json
import cmdHandler

BYTE_ORDER = 'little'


# Read a single byte
def readbyte(data, signed=True) -> (int, bytes):
    return readint(data, 1, signed)


# Read a two byte integer
def readshort(data, signed=True) -> int:
    return readint(data, 2, signed)


# Read integer and return the rest
def readint(data, len=4, signed=True) -> int:
    bytes, data = readbytes(data, len)
    byte = int.from_bytes(bytes, byteorder=BYTE_ORDER, signed=signed)
    return byte, data


# Read N bytes and return the rest of the array
def readbytes(data, len) -> (bytes, bytes):
    return data[:len], data[len:]


# Convert int to bytes
def int_to_bytes(num, len=4, signed=True) -> bytes:
    return num.to_bytes(len, byteorder=BYTE_ORDER, signed=signed)


# Read a string where the first byte is its length
def readstring(data, signature_len=1, encoding='utf-8') -> (str, bytes):
    length, data = readint(data, signature_len)
    string, data = readbytes(data, length)
    return string.decode(encoding), data


def writestring(string, signature_len=1, encoding='utf-8') -> (str, bytes):
    b = bytes(string, encoding)
    return int_to_bytes(len(b), signature_len, False) + b


# Read a generic object in format (id: 2 bytes), (len: 2 bytes), (obj, 4 + len bytes)
def readobject(data) -> (int, bytes, bytes):
    id, data_new = readint(data, 2)
    len, _ = readint(data_new, 2)
    obj, data = readbytes(data, len + 4)  # read id and len as well
    return id, obj, data


def readfloat(data):
    b, data = readbytes(data, 4)
    f = struct.unpack('f', b)[0]
    return f, data


def writefloat(f):
    return struct.pack('f', f)


# Unused. Will be used to log errors
def log(text, *args, **kwargs) -> None:
    """ type: (bytes) -> None
    """
    if cmdHandler.cmdHandler.debug:
        print(str(text).format(*args, **kwargs))
    # print(data, file="log.txt")
    pass


class Payload(object):
    def __init__(self, j, **kwargs):
        # self.__dict__ = kwargs
        if type(j) is str:
            self.__dict__ = json.loads(j)
        elif type(j) is dict:
            self.__dict__ = j
