import random

from binaryUtils import *
import threading
#import sqlalchemy


# The base class for all the tables and viewers


class Client:
    # Should never happen
    CORRUPT_PACKAGE = b'\x00CP'

    def __init__(self, sock):
        self.socket = sock  # type: ThreadedTCPRequestHandler
        self.opcode_to_function = {}  # type: dict[int, function]

    def handle_opcode(self, data):
        responses = []
        while len(data) > 0:
            opcode, data = readbyte(data)
            result = b"ER"
            if opcode in self.opcode_to_function:
                result, data = self.opcode_to_function[opcode](data)

            responses.append(int_to_bytes(opcode, 1) + result)

            # Check for \x00 in between different opcodes. Send error if not and remove it from data
            if len(data) > 0:
                safe_code, data = readbyte(data)
                if(safe_code != 0):
                    print (self, "received corrupted package.")
                    return Client.CORRUPT_PACKAGE

        # Send responses to all the oppcodes separated by \x00
        return b'\x00'.join(responses)

    def on_disconnect(self):
        pass


# Base class for a generic table
class Table(Client):
    opcode = 1

    def __init__(self, sock, table_id):
        super().__init__(sock)
        self.id = table_id  # Table's name
        self.lock = threading.Lock()  # Lock for multi-threading
        self.viewers = set()  # All the viewers. Currently not used and may be never used
        self.objects = {}  # type: dict[int, bytes]; desc: Full and current state
        self.updates = {}  # Supposed to store delta changes
        self.last_delta = 0
        self.last_full_update = 0
        # Contains pointers to functions for specified opcodes
        self.opcode_to_function = {1: self.initialize_data, 2: self.update_data}

    # Supposed to be called only once on initialization. Currently used instead of update data
    def initialize_data(self, data):
        n_objects, data = readint(data, 2)
        for i in range(n_objects):
            obj_id, obj, data = readobject(data)
            self.objects[obj_id] = obj

        self.last_delta += 1
        self.last_full_update = self.last_delta
        return b"OK", data

    # Supposed to take care of delta-changes
    def update_data(self, data):
        pass

    # Assign a viewer to the table
    def add_viewer(self, viewer):
        self.viewers.add(viewer)

    # Remove a viewer from the table
    def remove_viewer(self, viewer):
        if viewer in self.viewers:
            self.viewers.remove(viewer)

    def retrieve_data(self, last_delta):
        with self.lock:
            new_delta = self.last_delta

            if last_delta >= self.last_delta:
                return int_to_bytes(new_delta, 4) + b'\x00\x00'

            data = int_to_bytes(new_delta, 4) + int_to_bytes(len(self.objects), 2)
            for key in self.objects:
                obj = self.objects[key]
                dependent_data = self.get_dependent_data(key)
                data += obj + dependent_data
            return data

    def get_dependent_data(self, key):
        return b''


# Base class for a generic viewer
class Viewer(Client):
    opcode = 2

    def __init__(self, sock):
        super().__init__(sock)
        self.table = None
        self.opcode_to_function = {1: self.get_new_data}

    def get_new_data(self, data):
        last_delta, data = readint(data)

        package = self.table.retrieve_data(last_delta)
        return package, data


class CityIOApp(Viewer):
    opcode = 4

    def __init__(self, sock):
        super().__init__(sock)
        self.opcode_to_function[2] = self.get_status  # Args: None. Return: short(status)
        self.opcode_to_function[3] = self.post_comment  # Args: str(text), float(x), float(y), float(z), short(n), n*short(building_id). Returns: b'OK'
        self.opcode_to_function[4] = self.get_comments  # Args: short(object_id).
        # Returns: short(n), n*[int(cid), str(text), int(likes)]
        self.opcode_to_function[5] = self.get_random_comment # Args: None. Return id, text, x, yz
        self.opcode_to_function[6] = self.like_comment  # Args: int(comment_id). Returns: b'OK'

    def get_status(self, data):
        # Watch out for multithreading
        status = self.table.status
        return bytes([status]), data

    def post_comment(self, data):
        text, data = readstring(data, 2)
        x, data = readfloat(data)
        y, data = readfloat(data)
        z, data = readfloat(data)
        n, data = readshort(data)
        buildings = []
        for i in range(n):
            b_id, data = readshort(data)
            buildings.append(b_id)
        self.table.add_comment(buildings, text, x, y, z)
        return b'OK', data

    def get_random_comment(self, data):
        comment = self.table.get_random_comment()
        if comment is None:
            return int_to_bytes(0), data

        id, text, x, y, z = comment
        print(id, text, x, y, z)
        return int_to_bytes(id, 4) + writestring(text, 2) + writefloat(x) + writefloat(y) + writefloat(z), data

    def get_comments(self, data):
        building_id, data = readshort(data)
        comments = self.table.get_comments(building_id)
        new_list = [int_to_bytes(v.id)+writestring(comments.text, 2)+int_to_bytes(v.likes) for v in comments]
        response = int_to_bytes(len(comments), 2) + b''.join(new_list)
        return response, data

    def like_comment(self, data):
        comment_id, data = readint(data)
        self.table.like_comment(comment_id)
        return b'OK', data


# Table class specifically for the CityIO app
class CityIOTable(Table):
    opcode = 3

    class Comment:
        def __init__(self, comment_id, text, x, y, z):
            self.id = comment_id
            self.x = x
            self.y = y
            self.z = z
            self.text = text
            self.likes = 0

    class FeedbackInfo:
        def __init__(self):
            self.magnitude = 0
            self.comments = set()

        def add_comment(self, comment):
            self.magnitude += 1
            self.comments.add(comment)

    def __init__(self, sock, table_id):
        super().__init__(sock, table_id)

        # Status representing if we are receiving the real data or the fake changes
        self.status = False
        self.opcode_to_function[3] = self.update_status
        self.feedback_data = {}
        self.comments = {}
        self.comments_lastid = 0

    def get_feedback(self, b):
        return self.feedback_data.get(b, CityIOTable.FeedbackInfo())

    def get_dependent_data(self, key):
        if(0 <= key < 256):
            heat = self.get_feedback(key)
            return int_to_bytes(heat.magnitude, 2)

        return b''

    def update_status(self, data):
        status, data = readbyte(data)
        self.status = status == 1
        return b'OK', data

    def add_comment(self, buildings, text, x, y, z):
        with self.lock:
            self.comments_lastid += 1
            c = CityIOTable.Comment(self.comments_lastid, text, x, y, z)
            self.comments[c.id] = c
            for b in buildings:
                info = self.get_feedback(b)
                info.add_comment(c)

    def get_comments(self, building):
        with self.lock:
            return list(self.get_feedback(building).comments)

    def get_random_comment(self):
        if len(self.comments) == 0:
            return None
        c_id, c_data = random.choice(list(self.comments.items()))
        print(c_id, c_data, c_data.x, c_data.y, c_data.z)
        return c_id, c_data.text, c_data.x, c_data.y, c_data.z

    def like_comment(self, comment_id):
        with self.lock:
            if comment_id in self.comments:
                self.comments[comment_id].likes += 1


# Table for converting opcode to a client type
TYPE_BYTECODE = {Table.opcode: Table, Viewer.opcode: Viewer, CityIOTable.opcode: CityIOTable,
                 CityIOApp.opcode: CityIOApp}
