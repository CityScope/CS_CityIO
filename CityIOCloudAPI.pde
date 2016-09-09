import processing.net.*;
public class CityIOCloudAPI{

  
  PApplet applet;
  String ip;
  int port;
  int mode;
  
  String tablename;
  int width;
  int height;
  
  byte[] byteBuffer = new byte[100*1024];
  boolean isConnected = false;
  boolean isRequest1Sent = false;
  boolean isJSONValid = false;
  String curRecievedString = "";
  String curJSON;
  Client myClientMain;
  int displayWidth = 200;
  int displayHeight = 200;
  
  IntDict[][] lastGrid;
  ComparableJSONObject lastObjects;

  
  private CityIOCloudAPI(PApplet p, String ip, int port, String tablename, int width, int height) {
    applet = p;
    this.ip = ip;
    this.port = port;
    //this.mode = mode;
    this.tablename = tablename;
    this.width = width;
    this.height = height;
    
    myClientMain = new Client(p, ip, port);
    
  } 
  
  void connect() {
    isRequest1Sent = false;
    if(!myClientMain.active())
      myClientMain = new Client(applet, ip, port);
    if(myClientMain.active())
      connectToServerAsTable(tablename,width,height);
  }
  
  void connectToServerAsTable(String name, int width, int height) {
    JSONObject jobj = new JSONObject();
    jobj.setString("salt", "CIOMIT");
    jobj.setString("type", "table");
    jobj.setString("id", name);
    jobj.setInt("width", width);
    jobj.setInt("height", height);
    myClientMain.write(jobj.toString());
    waitConnectToserver();
    
    //SendRequest("init", data);
  }
  
  void waitConnectToserver(){
    //int recievedNumberOfBytes = 0;
    //int curByteCount = 0;
    String j = "";
    while (getValidJSonObject(j) == null){
      j += readCurrentBuffer();
    }
    isConnected = true;
  }
  
  void handleUpdate(JSONArray grid, ComparableJSONObject objects){
    if(!myClientMain.active()) {
      println("Trying to reconnect");
      connect();
      if(myClientMain.active()) {
        println("Reconnected successfully");
      } else {
        println("Reconnection unsuccessful");
        return;
      }
    }
    if(!isRequest1Sent){
      if(lastGrid == null) {
        JSONObject data = new JSONObject();
        data.setJSONArray("grid", grid);
        data.setJSONObject("objects",objects);
        SendRequest("init", data);
        saveGrid(grid);
        lastObjects = objects;
      } else {
        JSONObject data = calculateDelta(grid, objects);   
        if(data == null)
          return;
        SendRequest("update", data);
        saveGrid(grid);
        lastObjects = objects;
      }
      //curJSON = "";
    
    }
    else if(isRequest1Sent){
      receive();
    }
  }
  
  JSONObject calculateDelta(JSONArray grid, ComparableJSONObject objects) {
    JSONArray delta_grid = new JSONArray();
    for(int i = 0; i < grid.size(); i++) {
      JSONObject cell = grid.getJSONObject(i);
      int x = cell.getInt("x");
      int y = cell.getInt("y");
      int type = cell.getInt("type");
      int rot = cell.getInt("rot");
      IntDict oldCell = lastGrid[y][x];
      if(oldCell.get("type") != type || oldCell.get("rot") != rot) {
        delta_grid.append(cell);
      }    
    }
    ComparableJSONObject delta_objects = objects.getDifferences(lastObjects);
    
    if(delta_grid.size() == 0 && delta_objects.size() == 0)
      return null;
      
    JSONObject result = new JSONObject();
    result.setJSONArray("grid", delta_grid);
    result.setJSONObject("objects", delta_objects);
    return result;
  }
  
  void saveGrid(JSONArray grid) {
    lastGrid = new IntDict[height][width];
    for(int i = 0; i < grid.size(); i++) {
      JSONObject cell = grid.getJSONObject(i);
      int x = cell.getInt("x");
      int y = cell.getInt("y");
      int type = cell.getInt("type");
      int rot = cell.getInt("rot");
      IntDict d = new IntDict();
      d.set("type", type);
      d.set("rot", rot);
      lastGrid[y][x] = d;
    }
  }
  
  JSONObject receive() {
    
    String recv = readCurrentBuffer();
    if(recv.length() > 0) {
      
      curJSON += recv;
      //println(String.format("Received %d, total: %d", recv.length(), curJSON.length()));
      //println(curJSON);
      JSONArray jsonArray = getValidJSonArray(curJSON);
      if(jsonArray != null){
        //isJSONValid = true;
        isRequest1Sent = false;
        JSONObject json= null;
        for (int i = 0; i < jsonArray.size(); i++) {
            json = jsonArray.getJSONObject(0);
            //json.
            if(json.getInt("id", 0) == 1) {
              return json;
            }
        }
        return json;
      }
      else{
        isJSONValid= false;
        return null;
      }
    }
    return null;
    
  }
  
  JSONArray getValidJSonArray(String curString){
    try{
      JSONArray json = parseJSONArray(curString);
      return json;
    }
    catch (Exception e) {
      //e.printStackTrace();
     return null;
    }
  }
  
  JSONObject getValidJSonObject(String curString){
    try{
      JSONObject json = parseJSONObject(curString);
      return json;
    }
    catch (Exception e) {
      //e.printStackTrace();
     return null;
    }
  }
  
  
  
  void SendRequest(String opcode, JSONObject data){
    JSONArray jsonArrayRequest1 = new JSONArray();
    JSONObject jsonRequest1 = new JSONObject();
    jsonRequest1.setInt("id", 1);
    jsonRequest1.setString("opcode", opcode);
    jsonRequest1.setJSONObject("data", data);
    jsonArrayRequest1.setJSONObject(0, jsonRequest1);
    try{
      //myClientMain.active();
      myClientMain.write(jsonArrayRequest1.toString());
      isRequest1Sent = true;
    } catch (NullPointerException e) {
      println("Connection lost, trying to reconnect..");
      //this.myClientMain = new Client(applet, ip, port);
    }
    curJSON = "";
  }
  
  String readCurrentBuffer(){
    if (myClientMain.available() > 0) {
      int byteCount = myClientMain.readBytes(byteBuffer);
      String myString = new String(byteBuffer, 0, byteCount);
      return myString;
    } else return "";
  }
  

  public String bytesToHex(byte[] in, int count) {
      final StringBuilder builder = new StringBuilder();
      for(int i = 0; i < count; i++) {
          byte b = in[i];
          builder.append(String.format("%02x", b));
      }
      return builder.toString();
  }
}
