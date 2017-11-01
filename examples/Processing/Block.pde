class Block {
  PVector location;
  int size;
  int id;

  Block(PVector l, int _size, int _id) {
    location = l;//.copy();
    size = _size;
    id= _id;
  }
 
  void run(){
  }

  // Method to display
  void display(PGraphics p) { 
    p.rectMode(CENTER);
      p.fill(255);
      p.textSize(12);
      p.textAlign(CENTER);
      p.text(id,  location.x, location.y);
      if(id==6){
        p.fill(25);
        p.rect(location.x, location.y, size, size); 
      }
      if(id==-1){
        p.fill(125);
        p.rect(location.x, location.y, size, size); 
      }
      if(id==0 || id==1 || id ==2){
        p.fill(255,250,205);
        p.rect(location.x, location.y, size, size);
        if(id==0){
          p.fill(0,0,255);
          p.rect(location.x, location.y, size/4, size/4); 
        }
        if(id==1){
          p.fill(255,255,0);
          p.rect(location.x, location.y, size/4, size/4); 
        }
        if(id==2){
          p.fill(255,0,0);
          p.rect(location.x, location.y, size/4, size/4); 
        } 
      }
      if(id==3 || id==4 || id ==5){
        p.fill(255,255,255);
        p.rect(location.x, location.y, size, size);
        if(id==3){
          p.fill(0,0,255);
          p.rect(location.x, location.y, size/4, size/4); 
        }
        if(id==4){
          p.fill(255,255,0);
          p.rect(location.x, location.y, size/4, size/4); 
        }
        if(id==5){
          p.fill(255,0,0);
          p.rect(location.x, location.y, size/4, size/4); 
        } 
      }
  }
}