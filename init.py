# The main file and the only one that is supposed to be run from console.
# Starts the UDP/TCP emulator in parallel and starts the server in the main thread.
import threading
from http.server import HTTPServer

import debugUDP
from serverHandler import ThreadedTCPServer, http_handler_factory

from CmdHandler import CmdHandler

VERSION = "8/01/2016 2:40PM"

if __name__ == "__main__":
    # The host and the port for the server. Keep the host blank, so the clients can connect using the domain name.
    print("TCP Server. Version of", VERSION)
    HOST, PORT = "", 9997

    # Start the UDP/TCP emulator, which connects to localhost:PORT
    # debugUDP.init("localhost", PORT)

    # Initialize and start the server main server.
    server = ThreadedTCPServer((HOST, PORT))
    server_thread = threading.Thread(target=lambda: server.serve_forever())
    server_thread.daemon = True
    server_thread.start()

    server_address = ('', 80)
    httpd = HTTPServer(server_address, http_handler_factory(server))
    http_server_thread = threading.Thread(target=lambda: httpd.serve_forever())
    http_server_thread.daemon = True
    http_server_thread.start()

    CmdHandler(server).cmdloop()
