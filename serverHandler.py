import socket
import threading
import socketserver
import clients

from binaryUtils import *

# The beginning of the string for any handshake
SALT = bytes("CIOMIT", 'ascii')
SALT_LEN = len(SALT)


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
                    # print("[TCP.Server] Received from {client}.".format(client=type(client)))
                    response = client.handle_opcode(data)
                    self.request.sendall(response)
                    data = self.request.recv(4048)
        except ConnectionResetError:
            print("[TCP.Server] Connection by {} was reset.".format(client))
        except ConnectionAbortedError:
            print("[TCP.Server] Connection by {} was aborted.".format(client))
        finally:
            if client is clients.Client:
                client.on_disconnect()

    def handle_first_input(self, data):
        salt, data = readbytes(data, SALT_LEN)
        if salt != SALT:
            return None

        type, data = readbyte(data)
        if type not in clients.TYPE_BYTECODE:
            return None

        client_type = clients.TYPE_BYTECODE[type]

        result = None
        if issubclass(client_type, clients.Viewer):
            table_id, data = readstring(data)
            #print(self.server.tables)
            result = client_type(self)

            self.server.assign_to_table(result, table_id)

        elif issubclass(client_type, clients.Table):
            table_id, data = readstring(data)
            result = self.server.create_table(client_type, socket, table_id)

        return result


# Class for the TCP server
class ThreadedTCPServer(socketserver.ThreadingMixIn, socketserver.TCPServer):
    def __init__(self, ip_port):
        self.tables = {}
        self.lock = threading.Lock()
        super().__init__(ip_port, ThreadedTCPRequestHandler)

    def create_table(self, table_type, sock, table_id) -> clients.Table:
        with self.lock:
            table = table_type(sock, table_id)
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



