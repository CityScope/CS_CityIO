import cmd
from typing import Dict, Set
from clients import CityIOTableJSON
from serverHandler import ThreadedTCPServer


def log(text, *args, **kwargs) -> None:
    """ type: (bytes) -> None
    """
    if CmdHandler.debug:
        print(str(text).format(*args, **kwargs))
    # print(data, file="log.txt")


class CmdHandler(cmd.Cmd):
    """Simple command processor example."""
    debug = False

    def __init__(self, server):
        super().__init__()
        self.server = server  # type: ThreadedTCPServer

    def do_debug(self, status):
        global debug
        if status == "on":
            CmdHandler.debug = True
            print("Debug is on now.")
        elif status == "off":
            CmdHandler.debug = False
            print("Debug is off now")
        else:
            print("debug (on|off)")

    def do_exit(self, line):
        quit(0)

    def do_comments(self, line):
        args = line.split(" ")
        opp = args[0]
        if opp == "clear":
            for k, table in self.server.tables.items():
                with table.lock:
                    comments = table.comments # type: Set(CityIOTableJSON.Comment)
                    comments.clear()

        elif opp == "info":
            for k, table in self.server.tables.items():
                with table.lock:
                    l = len(table.comments)  # type: Dict[int, CityIOTableJSON.Comment]
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


# Unused. Will be used to log errors

