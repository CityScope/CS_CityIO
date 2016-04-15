import socketserver
import re
import socket
import time
import threading
import random
from binaryUtils import *

ex_start = re.compile('^gridIndex\t(?P<gridIndex>\d+?)\n')
ex_data = re.compile('((?P<id>-?\d+)\t(?P<x>-?\d+)\t(?P<y>-?\d+)\t(?P<rot>-?\d+)\n?)+?')
SAFE_TIME_SECONDS = 15
change_freq = 8


def get_data(data, is_working):
    ll = ex_data.findall(data)
    data = b"\x01" + int_to_bytes(len(ll), 2)
    for row in ll:
        type, x, y, rot = int(row[1]), int(row[2]), int(row[3]), int(row[4])
        id = x * 16 + y
        data += int_to_bytes(id, 2) + int_to_bytes(5, 2) + int_to_bytes(x, 1) \
                + int_to_bytes(y, 1) + int_to_bytes(type, 1) + int_to_bytes(rot, 2)

    status_req = b'\x03' + bytes([is_working])
    return b'\x00'.join([data, status_req])


def client_emulator(ip, port, server):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect((ip, port))
    last_data = ""
    try:
        message = b"CIOMIT" + b"\x03" + b"\x04tab1"
        sock.sendall(message)
        response = sock.recv(1024)
        print("[TCP.TEmu] Received by table: {}".format(response))
        while True:
            data, is_working = server.get_data()

            if data != last_data:
                message = get_data(data, is_working)
                sock.sendall(message)
                print("[TCP.TEmu] Table has sent some data")
                response = sock.recv(1024)
                print("[TCP.TEmu] Received by table: {}".format(response))
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
            return (self.data, self.working)

    def set_data(self, data):
        with self.lock:
            if ex_start.match(data):
                self.data = data
                self.data_time = time.time()
            else:
                print("[UDP.TEmu] ERROR: Received data didn't match.")

    def apply_random_change(self):
        if time.time() - self.last_random_change_time > change_freq:

            last_random_change_time = time.time()
            rows = ex_data.findall(self.data)
            if len(rows) == 0:
                print("[TCP.TEmu] Tried to apply a change on empty set. Data was loaded from file.")
                self.load_data_from_file()
                return

            r = random.choice(rows)
            components = r[0].split('\t')
            type = int(components[0])
            old_type = type
            while True:
                type = -1 if type != -1 else random.randint(1, 15)
                # 6 is not allowed
                if type != 6:
                    break
            n_s = '\t'.join([str(type), components[1], components[2], components[3]])
            print("[DEBUG] New type is", type, "Old was", old_type)
            print("[UDP.TEmu] Just applied some random change")
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
            print("[UDP.TEmu] Just saved the latest data")
            self.last_save_time = time.time()


def startUDP(ip, port):
    time.sleep(5)
    HOST, PORT = "", 9998
    server = MyUDPServer((HOST, PORT), MyUDPHandler)
    server.start_tcp(ip, port)
    print("[UDP.TEmu] UDP starting server...")
    server.serve_forever()


def init(ip, port):
    server_thread = threading.Thread(target=lambda: startUDP(ip, port))
    server_thread.daemon = True
    server_thread.start()
