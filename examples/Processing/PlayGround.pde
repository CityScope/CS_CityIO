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
  void updateGridJSON() {
    int y=-1;
    JSONArray gridsA = jsonCityIO.getJSONArray("grid");
    for (int i=0; i < gridsA.size(); i++) {
      println("gridsA.size()"+gridsA.size());
      //if the first value in gridsA is a JSONobject,
      //its a "grid":[{"data":{"solar":1389,"traffic":0,"wait":0},"rot":0,"type":-1,"x":0,"y":0} json type
      if ((gridsA.get(0) instanceof JSONObject)) { 
        JSONObject grid =  gridsA.getJSONObject(i);
        int rot = grid.getInt("rot");
        int type = grid.getInt("type");
        int x = grid.getInt("x");
        y = grid.getInt("y");
        grids.get(0).addBlock(new PVector(15-x, y), 20, type);
      } 
      //else if gridsA is a JSONarray full of int,
      //its a "grid":[-1,0,0,0,0,0,0,0,0,0,0,0] json style
      else if ((gridsA.get(0) instanceof Integer)) {
        int x=i%16;
        if (x==0) {
          y++;
        }     
        //unused: int rot = grid.getInt("rot");
        int type = gridsA.getInt(0);
        grids.get(0).addBlock(new PVector(15-x, y), 20, type);
      }
      //else, this json is a mess
      else {
        println ("JSon parsing failed");
      }
    }
  }
}
