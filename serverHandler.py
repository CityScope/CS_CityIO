import socket
import threading
import socketserver

from binaryUtils import *

# The beginning of the string for any handshake
SALT = bytes("CIOMIT", 'ascii')
SALT_LEN = len(SALT)


# The base class for all the tables and viewers
class Client:
    def __init__(self, sock):
        self.socket = sock  # type: ThreadedTCPRequestHandler
        self.opcode_to_function = {} # type: dict[int, function]

    def handle_opcode(self, data):
        opcode, n_data = readbyte(data)
        result = b"ER"
        if opcode in self.opcode_to_function:
            result = self.opcode_to_function[opcode](n_data)
        result = int_to_bytes(data[0], 1) + result
        return result

    def on_disconnect(self):
        pass

# Base class for a generic table
class Table(Client):
    opcode = 1

    def __init__(self, sock, table_id):
        super().__init__(sock)
        self.id = table_id
        self.lock = threading.Lock()
        self.viewers = set()
        self.objects = {}   # type: dict[int, bytes]
        self.updates = {}
        self.last_delta = 0
        self.last_full_update = 0
        self.opcode_to_function = {1: self.initialize_data, 2: self.update_data}

    def initialize_data(self, data):
        n_objects, data = readint(data, 2)
        for i in range(n_objects):
            id, obj, data = readobject(data)
            self.objects[id] = obj

        self.last_delta += 1
        self.last_full_update = self.last_delta
        return b"OK"

    def update_data(self, data):
        pass

    def add_viewer(self, viewer):
        self.viewers.add(viewer)

    def remove_viewer(self, viewer):
        del self.viewers[viewer]

    def retrieve_data(self, last_delta):
        new_delta = self.last_delta

        if last_delta >= self.last_delta:
            return int_to_bytes(new_delta, 4) + b'\x00\x00'

        with self.lock:
            data = int_to_bytes(new_delta, 4) + int_to_bytes(len(self.objects), 2)
            for key in self.objects:
                obj = self.objects[key]
                data += obj
            return data


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
        return package


# Table for converting opcode to a client type
TYPE_BYTECODE = {Table.opcode: Table, Viewer.opcode: Viewer}


# Class for handling incoming connections
class ThreadedTCPRequestHandler(socketserver.BaseRequestHandler):
    def finish(self):
        print("[TCP.Server] Client {} Disconnected".format(self.client_address))

    def handle(self):
        client = None
        try:
            data = self.request.recv(1024)

            print("[TCP.Server] Incoming connection.".format())
            client = self.handle_first_input(data)
            print("[TCP.Server] New", client, "has joined.")
            response = b"\x00OK"
            if client is not None:
                self.request.sendall(response)
                data = self.request.recv(4048)
                while len(data) > 0:
                    print("[TCP.Server] Received from {client}.".format(client=type(client)))
                    response = client.handle_opcode(data)
                    self.request.sendall(response)
                    data = self.request.recv(4048)
        except ConnectionResetError:
            print("[TCP.Server] Connection by {} was reset.".format(client))
        except ConnectionAbortedError:
            print("[TCP.Server] Connection by {} was aborted.".format(client))
        finally:
            if client is Client:
                client.on_disconnect()

    def handle_first_input(self, data):
        salt, data = readbytes(data, SALT_LEN)
        if salt != SALT:
            return None

        type, data = readbyte(data)
        if type not in TYPE_BYTECODE:
            return None

        client_type = TYPE_BYTECODE[type]

        result = None
        if client_type == Viewer:
            table_id, data = readstring(data)
            #print(self.server.tables)
            result = Viewer(self)

            self.server.assign_to_table(result, table_id)

        elif client_type == Table:
            table_id, data = readstring(data)
            result = self.server.create_table(socket, table_id)


        return result

# Class for the TCP server
class ThreadedTCPServer(socketserver.ThreadingMixIn, socketserver.TCPServer):
    def __init__(self, ip_port):
        self.tables = {}
        self.lock = threading.Lock()
        super().__init__(ip_port, ThreadedTCPRequestHandler)

    def create_table(self, sock, table_id) -> Table:
        with self.lock:
            table = Table(sock, table_id)
            self.tables[table.id] = table
        return table

    def assign_to_table(self, viewer, table_id):
        with self.lock:
            if table_id not in self.tables:
                return

            if viewer.table is not None:
                viewer.table.remove_viewer(viewer)
            self.tables[table_id].add_viewer(viewer)
            viewer.table = self.tables[table_id]



