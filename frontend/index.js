import "babel-polyfill";
import * as L from "leaflet";
window.$ = require("jquery");
import "bootstrap";
import * as legoIO from "/img/legoio.png";
import * as shadow from "/img/shadow.png";

////////////////////////////////////////////////////////////////////////////////////

var updateInterval = 1000;

async function getCityIO(cityIOurl) {
  // GET method
  return $.ajax({
    url: cityIOurl,
    dataType: "JSON",
    callback: "jsonData",
    type: "GET",
    success: function(d) {
      return d;
    },
    // or error
    error: function(e) {
      console.log("GET error: " + e.status.toString());
      infoDiv("GET error: " + e.status.toString());
    }
  });
}
function clearNames(url) {
  return url.toString().replace("https://cityio.media.mit.edu/api/table/", "");
}
////////////////////////////////////////////////////////////////////////////////////

async function getTables() {
  let tableArray = [];
  let cityIOurl = "http://cityio.media.mit.edu/api/tables/list";

  const tables = await getCityIO(cityIOurl);
  console.log(tables);
  infoDiv("Reading CityIO tables [at API + V2.0]");

  for (let thisTableName = 0; thisTableName < tables.length; thisTableName++) {
    let thisTable = await getCityIO(tables[thisTableName]);
    //check id API v2 [to replace with proper check later]
    if (thisTable.header) {
      infoDiv(
        thisTableName +
          " of " +
          tables.length +
          " CityScope tables: " +
          clearNames(tables[thisTableName]).link(tables[thisTableName]) +
          " || Clear Table".link(
            "http://cityio.media.mit.edu/api/table/clear/" +
              clearNames(tables[thisTableName])
          )
      );

      thisTable = thisTable.header;
      tableArray.push({
        url: tables[thisTableName],
        name: thisTable.name,
        lat: thisTable.spatial.latitude,
        lon: thisTable.spatial.longitude
      });
    }
  }

  makeMap(tableArray);
}
////////////////////////////////////////////////////////////////////////////////////

function makeMap(tablesArray) {
  infoDiv("----making map----");

  var map = L.map("map").setView([51.505, -0.09], 1);
  // setup the map API
  L.tileLayer(
    "http://api.mapbox.com/styles/v1/relnox/cjg1ixe5s2ubp2rl3eqzjz2ud/tiles/512/{z}/{x}/{y}?access_token=pk.eyJ1IjoicmVsbm94IiwiYSI6ImNqd2VwOTNtYjExaHkzeXBzYm1xc3E3dzQifQ.X8r8nj4-baZXSsFgctQMsg",
    {
      maxZoom: 15,
      minZoom: 2
    }
  ).addTo(map);
  //hide leaflet link
  document.getElementsByClassName(
    "leaflet-control-attribution"
  )[0].style.display = "none";
  document.getElementsByClassName("leaflet-top leaflet-left")[0].style.display =
    "none";
  //lock map to relevant area view
  map.setMaxBounds(map.getBounds());

  ///////////////Map icons///////////////////////
  // create a costum map icon [cityIO or non]
  var iconSize = 40;
  var IOIcon = L.icon({
    iconUrl: legoIO.default,
    iconSize: [iconSize, iconSize],
    iconAnchor: [0, 0],
    popupAnchor: [0, 0],
    shadowUrl: shadow.default,
    shadowSize: [iconSize, iconSize],
    shadowAnchor: [0, -20]
  });

  for (var i = 0; i < tablesArray.length; i++) {
    //clear names of tables
    let url = tablesArray[i].url;
    url = clearNames(url);

    //create map marker
    let marker = new L.marker([tablesArray[i].lon, tablesArray[i].lat], {
      icon: IOIcon
    })
      .bindPopup("CityScope " + url)
      .addTo(map);
    marker.properties = tablesArray[i];

    marker.on("mouseover", function() {
      this.openPopup();
    });
    marker.on("mouseout", function() {
      this.closePopup();
    });
    marker.on("click", function() {
      //pass the marker data to setup method
      modalSetup(marker);
      infoDiv("clicked " + url);
    });
  }

  // click event handler to creat a chart and show it in the popup
  async function modalSetup(m) {
    //get the divs for content
    var infoDiv = document.getElementById("infoDiv");
    //get the binded props
    let tableMeta = m.properties;
    //put prj name in div
    infoDiv.innerHTML = clearNames(m.properties.url);

    //stop update on modal close
    $("#modal").on("hide.bs.modal", function() {
      clearInterval(refreshIntervalId);
    });

    update(tableMeta.url);
    //start interval fix set interval that way:
    //http://onezeronull.com/2013/07/12/function-is-not-defined-when-using-setinterval-or-settimeout/
    var refreshIntervalId = setInterval(function() {
      update(tableMeta.url);
    }, updateInterval);
    //open up the modal
    $("#modal").modal("toggle");
  }
}

////////////////////////////////////////////////////////////////////////////////////

async function update(url) {
  const cityIOjson = await getCityIO(url);
  infoDiv("last update: " + new Date(cityIOjson.meta.timestamp));
  // only show table header
  let jsonMerge = {};
  $.extend(jsonMerge, cityIOjson.meta, cityIOjson.header);
  let cityIOjsonString = JSON.stringify(jsonMerge, null, 2);
  let threeDiv = document.getElementById("threeDiv");
  threeDiv.innerHTML = "";
  //
  output(syntaxHighlight(cityIOjsonString));
  //
  function output(inp) {
    threeDiv.appendChild(document.createElement("pre")).innerHTML = inp;
  }
}

//
function syntaxHighlight(json) {
  json = json
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
  return json.replace(
    /("(\\u[a-zA-Z0-9]{4}|\\[^u]|[^\\"])*"(\s*:)?|\b(true|false|null)\b|-?\d+(?:\.\d*)?(?:[eE][+\-]?\d+)?)/g,
    function(match) {
      var cls = "number";
      if (/^"/.test(match)) {
        if (/:$/.test(match)) {
          cls = "key";
        } else {
          cls = "string";
        }
      } else if (/true|false/.test(match)) {
        cls = "boolean";
      } else if (/null/.test(match)) {
        cls = "null";
      }
      return '<span class="' + cls + '">' + match + "</span>";
    }
  );
}
////////////////////////////////////////////////////////////////////////////////////
//make info div [on screen console] or add text to it
function infoDiv(text) {
  let d = document.getElementById("log");
  // clear div if too much text
  if (d.scrollHeight > 5000) {
    d.innerHTML = null;
  } else {
    d.innerHTML += text + "<p></p>";
    d.scrollTop = d.scrollHeight;
  }
  return;
}

//////////////////////////////////////////
// APP START
//////////////////////////////////////////
getTables();
