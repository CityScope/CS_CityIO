class PlayGround {
  PVector location;
  int width;
  int height;
  
  ArrayList<Grid> grids;
  PVector gridSize;
  
  PlayGround(PVector l, int w, int h){
    location=l;
    width=w;
    height=h;
    grids = new ArrayList();
    grids.add(new Grid(location,16,16,20));
  }
  
  void display(PGraphics p){
    if(isGridHasChanged){
      updateGridJSON();
      isGridHasChanged = false;
    }
    
    p.fill(255);  
    p.textSize(10);
    p.text("PlayGround",location.x-width/2,location.y-height*0.52);
    for(Grid g: grids){
      g.display(p);
    }
  }
    
  PVector getGridSize(){
    return new PVector(16,16);
  }
  void updateGridJSON(){
    JSONArray gridsA = jsonCityIO.getJSONArray("grid");
    print("size ");
    println(gridsA.size());
    for(int i=0; i < gridsA.size(); i++) {
      JSONObject grid =  gridsA.getJSONObject(i);
      int rot = grid.getInt("rot");
      int type = grid.getInt("type");
      int x = grid.getInt("x");
      int y = grid.getInt("y");
      int magnitude = grid.getInt("magnitude");
      grids.get(0).addBlock(new PVector(15-x, y), 20, type,magnitude);    
    }
  }
}