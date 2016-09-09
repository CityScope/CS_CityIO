import random
import socket
import threading
from typing import List, Dict, Any, AnyStr, Set, Optional, Tuple, Callable
from binaryUtils import *


# import sqlalchemy


class JSONClient:
    """
    The base class for all the tables and viewers
    operation = { 'id': 123123, 'opcode' : 'string', 'data': } -> string
    "["+operation1+","+operation2+"]
    """
    RESULT_OK = {"result": "OK"}
    RESULT_ERROR = {"result:" "error"}

    def __init__(self, sock):
        self.socket = sock  # Reference to the parent
        self.opcode_to_function = {}  # type: Dict[int, Callable[[Dict[str, Any]], Any]]

    def handle_opcode(self, requests):
        responses = []
        for request in requests:
            r_id = request["id"]
            opcode = request["opcode"]
            if opcode in self.opcode_to_function:
                result = {
                    "id": r_id,
                    "data": self.opcode_to_function[opcode](request.get("data", {}))
                }
                responses.append(result)

        # Send responses to all the opcodes separated by \x00
        return responses


class CityIOAppJSON(JSONClient):
    opcode = 5
    typecode = "client"

    def __init__(self, sock: socket.socket, table):
        super().__init__(sock)
        self.table = table  # type: CityIOTableJSON
        self.opcode_to_function = {
            "get_updates": self.get_new_data,
            "get_status": self.get_status,  # Args: None. Return: short(status)
            "post_comment": self.post_comment,
            "get_comments": self.get_comments,
            "get_rnd_comment": self.get_random_comment,  # Args: None. Return id, text, x, yz
            "like_comment": self.like_comment  # Args: int(comment_id). Returns: b'OK'
        }

    def get_new_data(self, data):
        last_delta = data.get("delta", 0)
        result = {}
        grid, objects, new_delta = self.table.retrieve_data(last_delta)

        result["new_delta"] = new_delta
        grid_list = []
        for pos, cell in grid.items():
            x, y = pos
            row = {
                "x": x,
                "y": y,
                "rot": cell.rot,
                "type": cell.type,
                "magnitude": len(cell.comments)
            }
            grid_list.append(row)
        result["grid"] = grid_list
        result["objects"] = objects
        return result

    def get_status(self, data):
        # Watch out for multithreading
        status = self.table.status
        return {"status": status}

    def post_comment(self, data):
        d = Payload(data)
        self.table.add_comment(d.buildings, d.text, d.x, d.y, d.z)
        return self.RESULT_OK

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
            comments = list(self.table.state.grid[y][x].comments)

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
        return self.RESULT_OK

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
            # self.magnitude = 0
            self.comments = set()

        def add_comment(self, comment):
            # self.magnitude += 1
            self.comments.add(comment)

    class State:
        MAX_DELTA_COUNT = 1000
        MAX_DELTA_CHANGES = 500

        class Update:
            def __init__(self, grid: List[Tuple[int, int]], objects: List[str]):
                self.objects = objects  # type: List[str]
                self.grid = grid  # type: List[Tuple[int, int]]

        def __init__(self, grid_width, grid_height, delta):
            self.first_delta = delta
            self.grid = [[CityIOTableJSON.StructureInfo(0, 0) for _ in range(grid_width)] for _ in range(grid_height)]
            self.objects = {}  # type: Dict[str, Any]
            self.updates = []  # type: List['CityIOTableJSON.State.Update']

        def generate_delta(self, last_delta) -> \
                Tuple[Dict[Tuple[int, int], 'CityIOTableJSON.StructureInfo'], Dict[str, Any]]:

            from CmdHandler import log

            if last_delta <= self.first_delta or self.first_delta - last_delta > self.MAX_DELTA_CHANGES:
                grid = {}
                for i in range(len(self.grid)):
                    for j in range(len(self.grid[i])):
                        grid[(j, i)] = self.grid[i][j]
                objects = self.objects
                return grid, objects
                # return self.grid, self.objects

            updated_grid = set()
            updated_objects = set()

            off_i = last_delta-self.first_delta
            log("Calculating delta: version {}/{}/{}, off_i {}", last_delta, self.first_delta, len(self.updates), off_i)
            # if off_i >= len(self.updates):
            #     return [], []

            for i in range(off_i, len(self.updates)):
                u = self.updates[i]
                updated_grid.update(u.grid)
                updated_objects.update(u.objects)

                # Debug
                # l = ["({}, {})".format(x, y) for x, y in u.grid]
                log("Update {}: {}, {}", i, u.grid, u.objects)

            grid = {(x, y): self.grid[y][x] for x, y in updated_grid}
            objects = {v: self.objects[v] for v in updated_objects}

            return grid, objects

    def __init__(self, sock: socket.socket, table_id: str, grid_width: int, grid_height: int):
        super().__init__(sock)
        self.id = table_id  # Table's name
        self.lock = threading.Lock()  # Lock for multi-threading
        self.viewers = set()  # All the viewers. Currently not used and may be never used
        self.grid_width = grid_width
        self.grid_height = grid_height
        self.state = None  # type: CityIOTableJSON.State
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
        self.make_state()
        grid = data["grid"]

        for cell in grid:
            p = Payload(cell)
            structure = self.state.grid[p.y][p.x]
            structure.rot = p.rot
            structure.type = p.type

        self.state.objects = data["objects"]
        # for k, v in objects.items():
        #     self.objects[k] = v

        return self.RESULT_OK

    # Supposed to take care of delta-changes
    def update_data(self, data) -> Dict[str, Any]:

        grid = data["grid"]  # type: List[Dict[str, int]]
        objects = data["objects"]  # type: Dict[str, Any]

        updated_objects = [k for k, v in objects.items()]
        updated_grid = [(v["x"], v["y"]) for v in grid]
        self.state.updates.append(self.state.Update(updated_grid, updated_objects))

        for cell in grid:
            p = Payload(cell)
            structure = self.state.grid[p.y][p.x]
            structure.rot = p.rot
            structure.type = p.type

        self.state.objects.update(objects)
        self.last_delta += 1

        return self.RESULT_OK

    # Assign a viewer to the table
    def add_viewer(self, viewer):
        self.viewers.add(viewer)

    # Remove a viewer from the table
    def remove_viewer(self, viewer):
        if viewer in self.viewers:
            self.viewers.remove(viewer)

    def retrieve_data(self, last_delta: int = 0) -> Tuple[Dict[Tuple[int, int], StructureInfo], Dict[str, Any], int]:
        with self.lock:
            new_delta = self.last_delta
            grid, objects = self.state.generate_delta(last_delta)
            return grid, objects, new_delta

    def update_status(self, data):
        status = data["status"]
        self.status = status
        return self.RESULT_OK

    def add_comment(self, buildings, text, x, y, z):
        with self.lock:
            self.comments_lastid += 1
            c = self.Comment(self.comments_lastid, text, x, y, z)
            self.comments[c.id] = c
            # Each building is {"x": x, "y": y}
            for b in buildings:
                x, y = b["x"], b["y"]
                info = self.state.grid[y][x]
                info.add_comment(c)

    def get_random_comment(self) -> (Comment, int):
        if len(self.comments) == 0:
            return None, 0
        c_id, c_data = random.choice(list(self.comments.items()))
        return c_data, len(self.comments)

    @DeprecationWarning
    def like_comment(self, comment_id):
        with self.lock:
            if comment_id in self.comments:
                self.comments[comment_id].likes += 1

    def make_state(self):
        self.last_delta += 1
        self.state = self.State(self.grid_width, self.grid_height, self.last_delta)


# Table for converting opcode to a client type
TYPE_BYTECODE = {CityIOAppJSON.opcode: CityIOAppJSON, CityIOTableJSON.opcode: CityIOTableJSON}
TYPE_CODE = {CityIOTableJSON.typecode: CityIOTableJSON, CityIOAppJSON.typecode: CityIOAppJSON}
