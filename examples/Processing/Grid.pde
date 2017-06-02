class Grid {
  ArrayList<Block> blocks;
  PVector origin;
  int w;
  int h;
  int blockSize;

  Grid(PVector location, int _w, int _h, int _blockSize) {
    origin = location;
    w=_w;
    h=_h;
    blockSize=_blockSize;
    blocks = new ArrayList();
  }
 
  void display(PGraphics p){     
      p.rectMode(CENTER);  
      p.fill(125); 
      //p.rect(origin.x,origin.y,w*blockSize, h*blockSize);
      p.pushMatrix();
      p.translate(origin.x - (blockSize*w)/2, origin.y - (blockSize*h)/2);   
      for (Block b: blocks){
        p.translate(b.location.x*blockSize,b.location.y*blockSize); 
        b.display(p);
        p.translate(-b.location.x*blockSize,-b.location.y*blockSize);
      }
      p.popMatrix();
      
  }
  
  void addBlock(PVector _location, int _blockSize, int _id, int _data){ 
    blocks.add(new Block(_location, _blockSize, _id, _data));     
  }
  


}

 

 