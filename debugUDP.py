import socketserver
import re
import socket
import time
import threading
from binaryUtils import *


ex_start = re.compile('^gridIndex\t(?P<gridIndex>\d+?)\n')
ex_data = re.compile('((?P<id>-?\d+)\t(?P<x>-?\d+)\t(?P<y>-?\d+)\t(?P<rot>-?\d+)\n?)+?')


def get_data(data):
    ll = ex_data.findall(data)
    data = b"\x01" + int_to_bytes(len(ll), 2)
    for row in ll:
        type, x, y, rot = int(row[1]), int(row[2]), int(row[3]), int(row[4])
        id = x * 16 + y
        data += int_to_bytes(id, 2) + int_to_bytes(5, 2) + int_to_bytes(x, 1) \
                + int_to_bytes(y, 1) + int_to_bytes(type, 1) + int_to_bytes(rot, 2)
    return data


def client_emulator(ip, port, server):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect((ip, port))
    last_data = b"";
    try:
        message = b"CIOMIT" + b"\x01" + b"\x04tab1"
        sock.sendall(message)
        response = sock.recv(1024)
        print("[TCP.TEmu] Received by table: {}".format(response))
        while True:
            data = server.get_data()
            if data != last_data:
                message = get_data(data)
                sock.sendall(message)
                print("[TCP.TEmu] Table has sent some data")
                response = sock.recv(1024)
                print("[TCP.TEmu DEBUG]", message)
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
        #print("{} wrote something".format(self.client_address[0]))

        string = data.decode('ascii')
        self.server.set_data(string)
        #print(data)
        #socket.sendto(data.upper(), self.client_address)


class MyUDPServer(socketserver.UDPServer):

    def __init__(self, server_address, RequestHandlerClass):
        super().__init__(server_address, RequestHandlerClass)
        self.lock = threading.Lock()
        self.data = ""

    def start_tcp(self, ip, port):
        print("[TCP.TEmu] Starting TCP client...")
        server_thread = threading.Thread(target=lambda: client_emulator(ip, port, self))
        server_thread.daemon = True
        server_thread.start()

    def get_data(self):
        with self.lock:
            return self.data

    def set_data(self, data):
        with self.lock:
            if ex_start.match(data):
                self.data = data


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


