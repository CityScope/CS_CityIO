import deadpixel.keystone.*;

public int displayWidth = 300;
public int displayHeight = 300;
public int playGroundWidth = 300;
public int playGroundHeight = 300;
int nbCols,nbRows,cellSize;

JSONObject jsonCityIO = new JSONObject();
String url= "https://cityio.media.mit.edu/api/table/virtual_table";
String local_file= "data/virtual_table.json";

PlayGround myPlayGround;
Keystone ks;
CornerPinSurface surface;
PGraphics offscreen;
boolean isGridHasChanged=true;

void setup() {
  size(displayWidth,displayWidth, P2D); //<>// //<>//
  ks = new Keystone(this);
  surface = ks.createCornerPinSurface(playGroundWidth,playGroundHeight,50); 
  offscreen = createGraphics(displayWidth, displayHeight, P2D);
  
  try{
    jsonCityIO = loadJSONObject(url);
  }
  catch(Exception e){
    println("The connexion to " + url + " failed");
    jsonCityIO = loadJSONObject(local_file);
  }
  initGridJSON();
  myPlayGround = new PlayGround(new PVector(playGroundWidth/2,playGroundHeight/2), playGroundWidth,playGroundWidth,nbCols,nbRows,cellSize);
  isGridHasChanged = true;
}
 
void draw() {
  background(255);
  offscreen.beginDraw();
  if (frameCount % 30 == 0){
    try{
      jsonCityIO = loadJSONObject(url);
    }
    catch(Exception e){
      println("The connexion to " + url + " failed");
      jsonCityIO = loadJSONObject(local_file);
    }
    isGridHasChanged = true;
  } 
  offscreen.clear();
  myPlayGround.display(offscreen);
  offscreen.endDraw();
  surface.render(offscreen); 
}

  void initGridJSON(){
    JSONObject gridsHeader = jsonCityIO.getJSONObject("header");
    JSONObject gridsHeaderSpatial = gridsHeader.getJSONObject("spatial");
    nbRows= gridsHeaderSpatial.getInt("nrows"); 
    nbCols = gridsHeaderSpatial.getInt("ncols"); 
    cellSize= gridsHeaderSpatial.getInt("cellsize");
    
  }  
  
  void updateGridJSON(){
    myPlayGround.grids.get(0).blocks.clear();
    JSONArray gridsA = jsonCityIO.getJSONArray("grid");
    for(int i= 0;i< nbCols;i++){
      for (int j=0;j<nbRows;j++){
        JSONArray grid =  gridsA.getJSONArray(j*nbCols+i);
        myPlayGround.grids.get(0).addBlock(new PVector(nbCols-i, j), cellSize, grid.getInt(0));
      }
    }
  }
  
  void exportGrid() {
    JSONObject mesh = new JSONObject();
    JSONObject metadata = new JSONObject();

    metadata.setString("id", "AOpifOF");
    metadata.setFloat("timestamp", millis());
    metadata.setString("apiv", "2.1.0");

    JSONObject header = new JSONObject();
    header.setString("name", "cityscope_processing");

    JSONObject spatial = new JSONObject();
    spatial.setInt("nrows", nbRows);
    spatial.setInt("ncols", nbCols);
    spatial.setFloat("physical_longitude", 5);
    spatial.setFloat("physical_latitude", 45);
    spatial.setInt("longitude", 5);
    spatial.setInt("latitude", 45);
    spatial.setInt("cellSize", cellSize);
    spatial.setInt("rotation", 0);

    header.setJSONObject("spatial", spatial);

    JSONObject owner = new JSONObject();
    owner.setString("name", "Arnaud Grignard");
    owner.setString("title", "Researcher Scientist");
    owner.setString("institute", "MIT Media Lab");
    
    header.setJSONObject("owner", owner);

    JSONArray block = new JSONArray();
    block.setString(1, "type");
    block.setString(0, "rotation");

    header.setJSONArray("block", block);

    JSONObject type = new JSONObject();
    JSONObject mapping = new JSONObject();
    type.setFloat("RL", 0);
    type.setFloat("RM", 1);
    type.setFloat("RS", 2);
    type.setFloat("OL", 3);
    type.setFloat("OM", 4);
    type.setFloat("OS", 5);
    type.setFloat("ROAD", 6);
    mapping.setJSONObject("type", type);

    header.setJSONObject("mapping", mapping);

    int k = 0;
    JSONArray grid = new JSONArray();
    for (int w = 0; w < myPlayGround.grids.get(0).blocks.size(); w++) {
      JSONArray arr = new JSONArray();
      arr.append(myPlayGround.grids.get(0).blocks.get(w).id);
      arr.append(0);
      arr.append(0);
      grid.append(arr);
    }

    header.setJSONArray("grid", grid);

    mesh.setJSONObject("meta", metadata);
    mesh.setJSONObject("header", header);

    saveJSONObject(mesh, "data/grid.json");
    //saveJSONObject(mesh, "https://cityio.media.mit.edu/api/table/update/cityIO_Processing");
    println("Grid Exported");
  }
  
  void keyPressed(KeyEvent e) {
    switch(key){
      case 'e':
      exportGrid();
      break;
    }
  }
