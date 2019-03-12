/**
* Name: cityIO API for GAMA
* Author: Arnaud Grignard
* Description: Gives the basic read and write feature for cityIO
*/
model CityIOGAMA



global {

	//geometry shape <- square(1 #km);
 	string cityIOurl <-"https://cityio.media.mit.edu/api/table/virtual_table"; 	
 	string VIRTUAL_LOCAL_DATA <- "./../includes/virtual_table.json";
    map<string, unknown> inputMatrixData;
    map<string, unknown> outputMatrixData;
    map<int,rgb> buildingColors <-[-2::#red, -1::#orange,0::rgb(189,183,107), 1::rgb(189,183,107), 2::rgb(189,183,107),3::rgb(230,230,230), 4::rgb(230,230,230), 5::rgb(230,230,230),6::rgb(40,40,40),7::#cyan,8::#green,9::#gray];
    map<string, unknown> header;
    map<string, unknown> spatial;
	int refresh <- 100 min: 1 max:1000 parameter: "Refresh Grid rate (cycle):" category: "Grid";
	bool pushToCityIO <- false parameter: "Push to cityIO every refresh cycles" category: "Grid";
	int nbCols;
	int nbRows;
	int cellSize;

	init {
	 do initGrid;
	}
    // Get a cityIO grid from a url and populate matrixData object
	action initGrid{
		try {
			inputMatrixData <- json_file(cityIOurl).contents;
		}

		catch {
			inputMatrixData <- json_file(VIRTUAL_LOCAL_DATA).contents;
			write #current_error + "Connection to Internet lost or cityIO is offline - CityMatrix is a local version from cityIO_Kendall.json";
		}
        
		spatial <-inputMatrixData["header"]["spatial"];
		loop i from: 0 to: nbCols-1 {
			loop j from: 0 to: nbRows -1{
				cityMatrix cell <- cityMatrix grid_at { i, j };
				cell.type<-int(inputMatrixData["grid"][j*nbCols+i][0]);
				cell.depth<-int(inputMatrixData["grid"][j*nbCols+i][1]);
			}
        }  
	}

	action pushGrid (map<string, unknown> _matrixData){
	  outputMatrixData <- _matrixData;
	  map(outputMatrixData["header"]["owner"])["institute"]<-"Gama Platform";
	  map(outputMatrixData["header"]["owner"])["name"]<-"Arnaud Grignard";
	  map(outputMatrixData["header"]["owner"])["title"]<-"Research Scientist";
	  save(json_file("https://cityio.media.mit.edu/api/table/update/cityIO_Gama", outputMatrixData));
	}

	reflex updateGrid when: ((cycle mod refresh) = 0){
		do initGrid;
		if(pushToCityIO){
		  do pushGrid(inputMatrixData);	
		}
 	}
}

grid cityMatrix width:nbCols height:nbRows {
	int size;
	int type;
	int depth;
    aspect base{
	  draw shape color:buildingColors[type] depth:depth;
	  draw string(type) color:#black border:#black at:{location.x,location.y,depth+1};		
	}
}


experiment Display  type: gui {
	action _init_ {
   		map<string, unknown> data;
   		try {
			data <- json_file(cityIOurl).contents;
		}

		catch {
			data <- json_file(VIRTUAL_LOCAL_DATA).contents;
			write #current_error + "Connection to Internet lost or cityIO is offline - CityMatrix is a local version from cityIO_Kendall.json";
		}
   		
   		
		create CityIOGAMA_model with: [nbCols::int(data["header"]["spatial"]["ncols"]), nbRows::int(data["header"]["spatial"]["nrows"]),cellSize::int(data["header"]["spatial"]["cellsize"]),inputMatrixData::data]{
			shape <-rectangle(nbCols*cellSize, nbRows*cellSize);
		}
	}

	output {	
		display cityMatrixView  type:opengl  background:#black {
			species cityMatrix aspect:base;
		}
	}
}