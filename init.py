# The main file and the only one that is supposed to be run from console.
# Starts the UDP/TCP emulator in parallel and starts the server in the main thread.

import serverHandler
import debugUDP

VERSION = "4/26/2016 3:09PM"

if __name__ == "__main__":
    # The host and the port for the server. Keep the host blank, so the clients can connect using the domain name.
    print("TCP Server. Version of", VERSION)
    HOST, PORT = "", 9999

    # Start the UDP/TCP emulator, which connects to localhost:PORT
    debugUDP.init("localhost", PORT)

    # Initialize and start the server main server.
    server = serverHandler.ThreadedTCPServer((HOST, PORT))
    server.serve_forever()

