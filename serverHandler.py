import socket
import threading
import socketserver
import clients
import cmdHandler

from binaryUtils import *

# The beginning of the string for any handshake
SALT = "CIOMIT"
SALT_LEN = len(SALT)


# Class for handling incoming connections
def validate_json(string):
    try:
        return json.loads(string)
    except json.JSONDecodeError:
        return None


class ThreadedTCPRequestHandler(socketserver.BaseRequestHandler):
    def finish(self):
        print("[TCP.Server] Client {} Disconnected".format(self.client_address))

    def handle(self):
        client = None
        try:
            # data = self.request.recv(1024)
            client = self.handle_first_input()
        except ConnectionResetError:
            print("[TCP.Server] Connection by {} was reset.".format(client))
        except ConnectionAbortedError:
            print("[TCP.Server] Connection by {} was aborted.".format(client))
        except ValueError as error:
            print("[TCP.Server] Value Error:", error)
        finally:
            if client is clients.JSONClient:
                client.on_disconnect()

    def main_loop(self, client):

        data = self.recv_json()
        while data is not None and len(data) > 0:
            # print("[TCP.Server] Received from {client}.".format(client=type(client)))

            log("[TCP.Server] <- {}: {}", type(client).__name__, data)
            response = client.handle_opcode(data)
            self.request.sendall(response)
            # print("[TCP.Server] Sending response({} bytes) to {}".format(len(response), client))
            log("[TCP.Server] -> {}: {}", type(client).__name__, response)
            data = self.recv_json()

        if data is None:
            raise ValueError("Data is none")
        if len(data) == 0:
            raise ValueError("Data is empty")

    def handle_first_input(self):
        data = self.recv_json()

        salt = data["salt"]
        if salt != SALT:
            return None

        type_code = data["type"]
        if type_code not in clients.TYPE_CODE:
            return None

        client_type = clients.TYPE_CODE[type_code]

        result = None
        if issubclass(client_type, clients.CityIOAppJSON):
            table_id = data["table"]
            # print(self.server.tables)

            table = self.server.assign_to_table(result, table_id)
            result = client_type(self, table)

        elif issubclass(client_type, clients.CityIOTableJSON):
            table_id = data["id"]
            dim_x, dim_y = data["width"], data["height"]
            table = client_type(self, table_id, dim_x, dim_y)
            self.server.create_table(table)
            result = table

        print("[TCP.Server] New", client_type.__name__, "has joined.")

        if result is not None:
            response_obj = {"result": "OK"}
            response = bytes(json.dumps(response_obj), "utf-8")
            self.request.sendall(response)
            self.main_loop(result)

        return result

    def recv_json(self, size=1024):
        data = b''
        json_object = None
        while json_object is None:
            tmp = self.request.recv(size)
            if len(tmp) == 0:
                return None
            data += tmp
            json_object = validate_json(bytes.decode(data))
        return json_object


# Class for the TCP server
class ThreadedTCPServer(socketserver.ThreadingMixIn, socketserver.TCPServer):
    def __init__(self, ip_port):
        self.tables = {}
        self.lock = threading.Lock()
        super().__init__(ip_port, ThreadedTCPRequestHandler)

    def create_table(self, table: clients.CityIOTableJSON) -> clients.CityIOTableJSON:
        with self.lock:
            self.tables[table.id] = table

    def assign_to_table(self, viewer, table_id):
        with self.lock:
            if table_id not in self.tables:
                return None
            table = self.tables[table_id]
            return table
