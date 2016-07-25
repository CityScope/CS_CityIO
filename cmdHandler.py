import cmd
import clients
import serverHandler

class cmdHandler(cmd.Cmd):
    """Simple command processor example."""
    debug = False

    def __init__(self, server: serverHandler.ThreadedTCPServer):
        super().__init__()
        self.server = server

    def do_debug(self, status):
        global debug
        if status == "on":
            cmdHandler.debug = True
            print("Debug is on now.")
        elif status == "off":
            cmdHandler.debug = False
            print("Debug is off now")
        else:
            print("debug (on|off)")

    def do_exit(self, line):
        quit()

    def do_comments(self, line):
        args = line.split(" ")
        opp = args[0]
        if opp == "clear":
            for k, table in self.server.tables.items():
                with table.lock:
                    comments = table.comments # type: set(clients.CityIOTableJSON.Comment)
                    comments.clear()

        elif opp == "info":
            for k, table in self.server.tables.items():
                with table.lock:
                    l = len(table.comments)  # type: dict[int, clients.CityIOTableJSON.Comment]
                    print("Table {}: {} comments".format(table.id, l))

        elif opp == "list":
            if len(args) != 2:
                print("ERROR: Expected 2 arguments.")
                return

            table_id = args[1]
            if table_id not in self.server.tables:
                print("Table '{}' was not found. Use \"comments info\" to list all the tables.".format(table_id))
                return
            table = self.server.tables[table_id]
            with table.lock:
                print("Printing {} comments for table {}".format(len(table.comments), table_id))
                for id, c in table.comments.items():
                    print("id {}: \"{}\"".format(c.id, c.text))

        elif opp == "delete":
            if len(args) != 3:
                print("ERROR: Expected 3 arguments.")
                return

            table_id = args[1]
            if table_id not in self.server.tables:
                print("Table '{}' was not found. Use \"comments info\" to list all the tables.".format(table_id))
                return
            table = self.server.tables[table_id]
            c_id = int(args[2])
            with table.lock:
                if c_id in table.comments:
                    del table.comments[c_id]
                    print("Comment {} was deleted.".format(c_id))

        elif opp == "lookup":
            if len(args) != 3:
                print("ERROR: Expected 3 arguments.")
                return

            table_id = args[1]
            if table_id not in self.server.tables:
                print("Table '{}' was not found. Use \"comments info\" to list all the tables.".format(table_id))
                return
            table = self.server.tables[table_id]
            substring = args[2]
            with table.lock:
                for id, c in table.comments.items():
                    if substring in c.text:
                        print("id {}: \"{}\"".format(c.id, c.text))

    def postloop(self):
        print()
