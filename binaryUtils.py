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
def readstring(data) -> (str, bytes):
    length, data = readbyte(data)
    string, data = readbytes(data, length)
    return str(string), data

# Read a generic object in format (id: 2 bytes), (len: 2 bytes), (obj, 4 + len bytes)
def readobject(data) -> (int, bytes, bytes):
    id, data_new = readint(data, 2)
    len, _ = readint(data_new, 2)
    obj, data = readbytes(data, len + 4)  # read id and len as well
    return id, obj, data

# Unused. Will be used to log errors
def log(msg, data) -> None:
    """ type: (bytes) -> None
    """
    # print(data, file="log.txt")
    pass
