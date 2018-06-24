import deadpixel.keystone.*;

public int displayWidth = 500;
public int displayHeight = 500;
public int playGroundWidth = 500;
public int playGroundHeight = 500;

JSONObject jsonCityIO = new JSONObject();
String url= "https://cityio.media.mit.edu/api/table/citymatrix_volpe";

PlayGround myPlayGround;
Keystone ks;
CornerPinSurface surface;
PGraphics offscreen;
boolean isGridHasChanged=true;

void setup() {
  size(displayWidth,displayWidth, P2D); //<>//
  ks = new Keystone(this);
  surface = ks.createCornerPinSurface(playGroundWidth,playGroundHeight,50); 
  offscreen = createGraphics(displayWidth, displayHeight, P2D);
  myPlayGround = new PlayGround(new PVector(playGroundWidth/2,playGroundHeight/2), playGroundWidth,playGroundWidth);
  jsonCityIO = loadJSONObject(url);//.getJSONArray("grid");
  isGridHasChanged = true;
}
 
void draw() {
  background(255);
  offscreen.beginDraw();
  if (frameCount % 30 == 0){
    jsonCityIO = loadJSONObject(url);
    isGridHasChanged = true;
  } 
  offscreen.clear();
  myPlayGround.display(offscreen);
  offscreen.endDraw();
  surface.render(offscreen); 
}