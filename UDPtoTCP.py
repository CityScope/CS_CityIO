import socketserver
import re
import socket
import time
import threading
import random
import json
#from binaryUtils import *

# ex_start = re.compile('^gridIndex\t(?P<gridIndex>\d+?)\n')

ex_start = re.compile('^COLORTIZER\n')
ex_data = re.compile('(^(?P<id>-?\d+)\t(?P<x>-?\d+)\t(?P<y>-?\d+)\t(?P<rot>-?\d+))+?$', re.MULTILINE)
SAFE_TIME_SECONDS = 15
change_freq = 8


def get_data(text_data, is_working):
    ll = ex_data.findall(text_data)

    grid = []
    for row in ll:
        b_type, x, y, rot = int(row[1]), int(row[2]), int(row[3]), int(row[4])
        grid.append({"x": x, "y": y, "type": b_type, "rot": rot})

    lines = text_data.split('\n')
    density = [int(v) for v in lines[-2].split("\t")]
    population = [int(v) for v in lines[-1].split("\t")]

    # print("[debug] density: {}", density)

    # print("[debug] population: {}", population)
    # print("{0} + {1} ({2} + {3})", len(ll), 2, len(density), len(population))

    objects = {"population": population, "density": density}

    data = {"grid": grid, "objects": objects}
    init_request = {"id": 1, "opcode": "init", "data": data}
    status_request = {"id": 2, "opcode": "status", "data": {"status": is_working}}
    result = json.dumps([init_request, status_request])
    return result


def client_emulator(ip, port, server):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect((ip, port))
    last_data = ""
    try:
        conn_obj = {
            "salt": "CIOMIT",
            "type": "table",
            "id": "tab1",
            "width": 16,
            "height": 16
        }
        # message = b"CIOMIT" + b"\x06" + b"\x04tab1"
        message = bytes(json.dumps(conn_obj), "utf-8")
        sock.sendall(message)
        response = sock.recv(1024)
        # print("[TCP.TEmu] Received by table: {}".format(response))
        while True:
            data, is_working = server.get_data()

            if data != last_data:
                message = bytes(get_data(data, is_working), 'utf-8')
                sock.sendall(message)
                # print("[TCP.TEmu] Table has sent some data")
                response = sock.recv(1024)
                # print("[TCP.TEmu] Received by table: {}".format(response))
                last_data = data

            time.sleep(0.2)
    finally:
        print("[TCP.TEmu] Client shutting down")
        sock.close()


class MyUDPHandler(socketserver.BaseRequestHandler):
    """
    This class works similar to the TCP handler class, except that
    self.request consists of a pair of data and client socket, and since
    there is no connection the client address must be given explicitly
    when sending data back via sendto().
    """

    def handle(self):
        data = self.request[0].strip()
        socket = self.request[1]
        # print("{} wrote something".format(self.client_address[0]))

        string = data.decode('ascii')
        self.server.set_data(string)
        # print(data)
        # socket.sendto(data.upper(), self.client_address)


class MyUDPServer(socketserver.UDPServer):

    save_interval = 60
    file_name = 'CityIO.table.data.txt'

    def __init__(self, server_address, request_handler_class):
        super().__init__(server_address, request_handler_class)
        self.lock = threading.Lock()
        self.data = ""
        self.data_time = time.time()
        self.last_random_change_time = 0
        self.last_save_time = 60
        self.working = True

    def start_tcp(self, ip, port):
        server_thread = threading.Thread(target=lambda: client_emulator(ip, port, self))
        server_thread.daemon = True
        server_thread.start()

    def get_data(self):
        with self.lock:
            self.working = time.time() - self.data_time < SAFE_TIME_SECONDS
            if not self.working:
                self.apply_random_change()
            else:
                self.save_data_to_file()
            return self.data, self.working

    def set_data(self, data):
        with self.lock:
            if ex_start.match(data):
                self.data = data
                self.data_time = time.time()
            else:
                print("[UDP.TEmu] ERROR: Received data didn't match.")
                print("[UDP.TEmu] DEBUG:", data)

    def apply_random_change(self):
        if time.time() - self.last_random_change_time > change_freq:

            rows = ex_data.findall(self.data)
            if len(rows) == 0:
                print("[TCP.TEmu] Tried to apply a change on empty set. Data was loaded from file.")
                self.load_data_from_file()
                return

            r = random.choice(rows)
            components = r[0].split('\t')
            type = int(components[0])
            old_type = type
            #while True:
            type = random.randint(-1, 0)
                # 6 is not allowed
                # if type != 6:
                #    break
            n_s = '\t'.join([str(type), components[1], components[2], components[3]])
            # print("[DEBUG] New type is", type, "Old was", old_type)
            self.data = self.data.replace(r[0], n_s)
            self.last_random_change_time = time.time()

    def load_data_from_file(self):
        f = open(MyUDPServer.file_name, 'r')
        self.data = f.read()
        f.close()

    def save_data_to_file(self):
        if time.time() - self.last_save_time > MyUDPServer.save_interval and self.data != "":
            f = open(MyUDPServer.file_name, 'w')
            f.write(self.data)
            f.close()
            self.last_save_time = time.time()


def start_udp(ip, port):
    time.sleep(5)
    host_local, port_local = "", 9998
    server = MyUDPServer((host_local, port_local), MyUDPHandler)
    server.start_tcp(ip, port)
    print("[UDP.TEmu] UDP starting server...")
    server.serve_forever()


def init(ip, port):
    server_thread = threading.Thread(target=lambda: start_udp(ip, port))
    server_thread.daemon = True
    server_thread.start()