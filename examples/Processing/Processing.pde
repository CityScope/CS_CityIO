import deadpixel.keystone.*;

public int displayWidth = 500;
public int displayHeight = 500;
public int playGroundWidth = 500;
public int playGroundHeight = 500;
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
