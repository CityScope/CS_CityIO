import threading
import socketserver
import json
import re
from typing import List, Dict, Any, AnyStr, Set, Optional, Tuple, Callable, Union

# The beginning of the string for any handshake
from http.server import BaseHTTPRequestHandler

from clients import JSONClient, TYPE_CODE, CityIOAppJSON, CityIOTableJSON

SALT = "CIOMIT"
SALT_LEN = len(SALT)


def validate_json(string):
    """Shortcut to check if the string is a valid JSON without raising an exception."""
    try:
        return json.loads(string)
    except json.JSONDecodeError:
        return None


class ThreadedTCPRequestHandler(socketserver.BaseRequestHandler):
    """
    Class for the incoming clients (both Tables and Clients!). Takes care of the handshake and handling
    the request-response loop.
    """
    def __init__(self, request, client_address, server):
        super().__init__(request, client_address, server)
        self.client = None

    def finish(self):
        """The function is being called after the connection is reset."""
        print("[TCP.Server] Client {} Disconnected".format(self.client_address))

    def handle(self):
        """The function is being called after the connection has been installed."""
        # Catch exceptions to check if the connection is lost
        try:
            # Handle handshake
            self.client, error_msg = self.handle_first_input()
            # If the client is successfully initialized
            if self.client is not None:
                # Send OK code
                response_obj = {"result": "OK"}
                self.send_obj(response_obj)
                # Start main loop
                self.main_loop()
            else:
                # Send the error msg
                self.send_obj({"error:", error_msg})
        except ConnectionResetError:
            print("[TCP.Server] Connection by {} was reset.".format(self.client))
        except ConnectionAbortedError:
            print("[TCP.Server] Connection by {} was aborted.".format(self.client))
        except ValueError as e:
            print("[TCP.Server] Value Error:", e)
        finally:
            pass

    def main_loop(self):
        """Handles request/response loop"""
        from CmdHandler import log
        data = self.recv_json()
        client_name = type(self.client).__name__

        # While we keep receiving data through network
        while data is not None and len(data) > 0:
            log("[TCP.Server] <- {}: {}", client_name, data)
            response = self.client.handle_opcode(data)
            self.send_obj(response)
            log("[TCP.Server] -> {}: {}", client_name, json.dumps(response))
            data = self.recv_json()

        if data is None:
            raise ValueError("Data is none")
        if len(data) == 0:
            raise ValueError("Data is empty")

    def handle_first_input(self) -> Tuple[JSONClient, Union[str, None]]:
        """Handles the handshake."""
        data = self.recv_json()

        salt = data.get("salt", None)
        if salt != SALT:
            return None, "Salt did not match."

        type_code = data["type"]
        if type_code not in TYPE_CODE:
            return None, "Unknown type of the client"

        client_type = TYPE_CODE[type_code]

        client = None
        if issubclass(client_type, CityIOAppJSON):
            table_id = data["table"]
            # print(self.server.tables)

            table = self.server.assign_to_table(client, table_id)
            client = client_type(self, table)

        elif issubclass(client_type, CityIOTableJSON):
            table_id = data["id"]
            dim_x, dim_y = data["width"], data["height"]
            if table_id in self.server.tables:
                table = self.server.tables[table_id]
            else:
                table = client_type(self, table_id, dim_x, dim_y)
                self.server.create_table(table)
            client = table
        print("[TCP.Server] New", client_type.__name__, "has joined.")

        return client, "Given type is not supported"

    def send_obj(self, obj) -> None:
        """Shortcut to send a serialized object as a response."""
        self.send_string(json.dumps(obj))

    def send_string(self, string) -> None:
        """Shortcut to send a string as a response. Should not be used!"""
        response = bytes(string, "utf-8")
        self.request.sendall(response)

    def recv_json(self, size=1024) -> Union[Dict[str, object], List[Dict[str, object]]]:
        """Shortcut to receive an expected JSON object."""
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
    """
    Class for the server itself. Should contain the per-server variables like the list of the tables
    and database handler.
    """
    def __init__(self, ip_port):
        self.tables = {}  # type: Dict[str, CityIOTableJSON]
        self.lock = threading.Lock()
        super().__init__(ip_port, ThreadedTCPRequestHandler)

    def create_table(self, table: CityIOTableJSON) -> CityIOTableJSON:
        with self.lock:
            self.tables[table.id] = table

    def assign_to_table(self, viewer, table_id):
        with self.lock:
            if table_id not in self.tables:
                return None
            table = self.tables[table_id]
            return table


def http_handler_factory(server):
    """
    Factory for the HTTP handler.
    :param server:
    :return:
    """
    ex_path = re.compile('^/(?P<table_id>\w+).json(?P<delta>\?d=\d+)?$')

    class HTTPHandler(BaseHTTPRequestHandler):
        # def __init__(self, request, client_address, server):
            # super().__init__(request, client_address, server)
        def _set_headers(self):
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()

        def do_GET(self):
            self._set_headers()
            result = ex_path.match(self.path)
            if result is None:
                response_obj = {
                    "tables": list(server.tables.keys())
                }
            else:
                table_id = result.group("table_id")
                if table_id not in server.tables:
                    response_obj = {
                        "error": "table not found"
                    }
                else:
                    table = server.tables[table_id] # type: CityIOTableJSON
                    last_delta = int(result.group("delta"))
                    if last_delta is None:
                        last_delta = 0

                    grid, objects, new_delta = table.retrieve_data(last_delta)

                    grid_list = []
                    for pos, cell in grid.items():
                        x, y = pos
                        row = {
                            "x": x,
                            "y": y,
                            "rot": cell.rot,
                            "type": cell.type,
                            # "magnitude": len(cell.comments)
                        }
                        grid_list.append(row)

                    result = {
                        "new_delta": new_delta,
                        "grid": grid_list,
                        "objects": objects
                    }
                    response_obj = result

            response = bytes(json.dumps(response_obj), "utf-8")
            self.wfile.write(response)

        def do_HEAD(self):
            self._set_headers()

        def do_POST(self):
            # Doesn't do anything with posted data
            self._set_headers()
            # self.wfile.write("<html><body><h1>POST!</h1></body></html>")

    return HTTPHandler
