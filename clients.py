import random
import json
import socket

from binaryUtils import *
import threading


# import sqlalchemy


# The base class for all the tables and viewers
# operation = { 'id': 123123, 'opcode' : 'string', 'data': } -> string
# "["+operation1+","+operation2+"]"
class JSONClient:
    RESULT_OK = {"result": "OK"}
    RESULT_ERROR = {"result:" "error"}

    def __init__(self, sock):
        self.socket = sock  # type: ThreadedTCPRequestHandler
        self.opcode_to_function = {}  # type: dict[int, function]

    def handle_opcode(self, requests):
        responses = []
        # string = bytes.decode(data)
        # print(data)
        # print(string)
        # requests = json.loads(string)
        for request in requests:
            r_id = request["id"]
            opcode = request["opcode"]
            if opcode in self.opcode_to_function:
                result = {
                    "id": r_id,
                    "data": self.opcode_to_function[opcode](request.get("data", None))
                }
                responses.append(result)

        # Send responses to all the opcodes separated by \x00
        return bytes(json.dumps(responses), 'utf-8')


class CityIOAppJSON(JSONClient):
    opcode = 5

    typecode = "client"

    def __init__(self, sock: socket.socket, table):
        super().__init__(sock)
        self.table = table  # type: CityIOTableJSON
        self.opcode_to_function = {
            "get_updates": self.get_new_data,
            "get_status": self.get_status,  # Args: None. Return: short(status)
            "post_comment": self.post_comment,  # Args: str(text), float(x), float(y), float(z), short(n), n*short(building_id). Returns: b'OK'
            "get_comments": self.get_comments,  # Args: short(object_id). Returns: short(n), n*[int(cid), str(text), int(likes)]
            "get_rnd_comment": self.get_random_comment,  # Args: None. Return id, text, x, yz
            "like_comment": self.like_comment  # Args: int(comment_id). Returns: b'OK'
        }

    def get_new_data(self, data):
        last_delta = data["delta"]
        result = {}
        grid, new_delta = self.table.retrieve_data(last_delta)

        result["new_delta"] = new_delta
        grid_list = []
        if grid is not None:
            for i in range(len(grid)):
                for j in range(len(grid[i])):
                    cell = grid[i][j]
                    row = {
                        "x": j,
                        "y": i,
                        "rot": cell.rot,
                        "type": cell.type,
                        "magnitude": len(cell.comments)
                    }
                    grid_list.append(row)
        result["grid"] = grid_list
        result["objects"] = self.table.objects
        return result

    def get_status(self, data):
        # Watch out for multithreading
        status = self.table.status
        return {"status": status}

    def post_comment(self, data):
        d = Payload(data)
        self.table.add_comment(d.buildings, d.text, d.x, d.y, d.z)
        return JSONClient.RESULT_OK

    def get_random_comment(self, data):
        comment, total_number = self.table.get_random_comment()
        if comment is None:
            return None

        return {
            "text": comment.text,
            "x": float(comment.x),
            "y": float(comment.y),
            "z": float(comment.z),
            "id": comment.id,
            "total": total_number
        }

    def get_comments(self, data):
        result = []
        x = data.get("x", None)
        y = data.get("y", None)
        if x is None or y is None:
            comments = list(self.table.comments.values())
        else:
            comments = list(self.table.grid[y][x].comments)

        for comment in comments:
            info = {
                "text": comment.text,
                "x": float(comment.x),
                "y": float(comment.y),
                "z": float(comment.z),
                "id": comment.id
            }
            result.append(info)
        return result

    def like_comment(self, data):
        comment_id = data["comment_id"]
        self.table.like_comment(comment_id)
        return JSONClient.RESULT_OK

    # Each request packet is in the form [request1, request2, ..., requestN]
    # Each request is in the form {'id': id, 'opcode': operation, 'data': operation_specific_data}
    # Each answer packet is in the form [answer1, answer2, ..., answerN]
    # Each answer is in the form {'id': id, 'data': answer_specific_data}


# Table class specifically for the CityIO app
class CityIOTableJSON(JSONClient):
    """

    """
    opcode = 6
    typecode = "table"

    class Comment:
        def __init__(self, comment_id, text, x, y, z):
            self.id = comment_id
            self.x = x
            self.y = y
            self.z = z
            self.text = text
            self.likes = 0

    class StructureInfo:
        def __init__(self, b_type, b_rot):
            self.rot = b_rot
            self.type = b_type
            self.magnitude = 0
            self.comments = set()

        def add_comment(self, comment):
            # self.magnitude += 1
            self.comments.add(comment)

    def __init__(self, sock: socket.socket, table_id: str, grid_width: int, grid_height: int):
        super().__init__(sock)
        self.id = table_id  # Table's name
        self.lock = threading.Lock()  # Lock for multi-threading
        self.viewers = set()  # All the viewers. Currently not used and may be never used
        self.grid_width = grid_width
        self.grid_height = grid_height
        self.grid = [[CityIOTableJSON.StructureInfo(0, 0) for x in range(grid_width)] for y in range(grid_height)]
        self.objects = {}  # type: dict[int, bytes]; desc: Full and current state
        self.updates = {}  # Supposed to store delta changes
        self.last_delta = 0
        self.last_full_update = 0
        # Status representing if we are receiving the real data or the fake changes
        self.status = False
        self.feedback_data = {}
        self.comments = {}
        self.comments_lastid = 0

        # Contains references to functions for specified opcodes
        self.opcode_to_function = {
            "init": self.initialize_data,
            "update": self.update_data,
            "status": self.update_status
        }

    # Supposed to be called only once on initialization. Currently used instead of update data
    def initialize_data(self, data):
        grid = data["grid"]

        for cell in grid:
            p = Payload(cell)
            structure = self.grid[p.y][p.x]
            structure.rot = p.rot
            structure.type = p.type

        objects = data["objects"]
        for k, v in objects.items():
            self.objects[k] = v

        self.last_delta += 1
        self.last_full_update = self.last_delta
        return JSONClient.RESULT_OK

    # Supposed to take care of delta-changes
    def update_data(self, data):
        pass

    # Assign a viewer to the table
    def add_viewer(self, viewer):
        self.viewers.add(viewer)

    # Remove a viewer from the table
    def remove_viewer(self, viewer):
        if viewer in self.viewers:
            self.viewers.remove(viewer)

    def retrieve_data(self, last_delta: int = 0):# -> (list[list[StructureInfo]], int):
        with self.lock:
            new_delta = self.last_delta
            if last_delta >= self.last_delta:
                return [], new_delta
            return self.grid, new_delta

    def update_status(self, data):
        status = data["status"]
        self.status = status
        return JSONClient.RESULT_OK

    def add_comment(self, buildings, text, x, y, z):
        with self.lock:
            self.comments_lastid += 1
            c = CityIOTableJSON.Comment(self.comments_lastid, text, x, y, z)
            self.comments[c.id] = c
            # Each building is {"x": x, "y": y}
            for b in buildings:
                x, y = b["x"], b["y"]
                info = self.grid[y][x]
                info.add_comment(c)

    def get_random_comment(self) -> Comment:
        if len(self.comments) == 0:
            return None, 0
        c_id, c_data = random.choice(list(self.comments.items()))
        return c_data, len(self.comments)

    def like_comment(self, comment_id):
        with self.lock:
            if comment_id in self.comments:
                self.comments[comment_id].likes += 1


# Table for converting opcode to a client type
TYPE_BYTECODE = {CityIOAppJSON.opcode: CityIOAppJSON, CityIOTableJSON.opcode: CityIOTableJSON}
TYPE_CODE = {CityIOTableJSON.typecode: CityIOTableJSON, CityIOAppJSON.typecode: CityIOAppJSON}