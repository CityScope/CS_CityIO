import serverHandler
import debugUDP

if __name__ == "__main__":
    # Port 0 means to select an arbitrary unused port



    HOST, PORT = "", 9999

    debugUDP.init("localhost", PORT)

    server = serverHandler.ThreadedTCPServer((HOST, PORT))
    ip, port = server.server_address

    server.serve_forever()

