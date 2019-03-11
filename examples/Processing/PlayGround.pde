class PlayGround {
  PVector location;
  int width;
  int height;
  
  ArrayList<Grid> grids;
  PVector gridSize;
  
  PlayGround(PVector l, int w, int h,int nbCols,int nbRows,int cellSize){
    location=l;
    width=w;
    height=h;
    grids = new ArrayList();
    grids.add(new Grid(location,nbCols,nbRows,cellSize));
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
}
