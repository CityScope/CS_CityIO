# The main file and the only one that is supposed to be run from console.
# Starts the UDP/TCP emulator in parallel and starts the server in the main thread.
import threading
from http.server import HTTPServer
from serverHandler import ThreadedTCPServer, http_handler_factory

from CmdHandler import CmdHandler

VERSION = "8/01/2016 2:40PM"

if __name__ == "__main__":
    # The host and the port for the server. Keep the host blank, so the clients can connect using the domain name.
    print("TCP Server. Version of", VERSION)
    HOST, PORT = "", 9997

    # Start the UDP/TCP emulator, which connects to localhost:PORT
    # debugUDP.init("localhost", PORT)

    # Initialize the server main server.
    server = ThreadedTCPServer((HOST, PORT))
    # Start a new thread for the server to run
    server_thread = threading.Thread(target=lambda: server.serve_forever())
    # Attach thread to the main process
    server_thread.daemon = True
    # Start the thread
    server_thread.start()

    # Define HTTP address and port
    http_server_address = ('', 80)
    # Initialize the HTTP Server class. Use factory pattern to pass the TCP server instance to the HTTP server
    httpd = HTTPServer(http_server_address, http_handler_factory(server))
    # Creating and starting a thread
    http_server_thread = threading.Thread(target=lambda: httpd.serve_forever())
    http_server_thread.daemon = True
    http_server_thread.start()

    # Starting the command handler
    CmdHandler(server).cmdloop()
