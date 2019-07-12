import "babel-polyfill";
import * as L from "leaflet";
window.$ = require("jquery");
import "bootstrap";
import * as legoIO from "/img/legoio.png";
import * as shadow from "/img/shadow.png";

////////////////////////////////////////////////////////////////////////////////////

var updateInterval = 2000;

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
  let cityIOurl = "https://cityio.media.mit.edu/api/tables/list";
  const tables = await getCityIO(cityIOurl);
  console.log(tables);

  for (let i = 0; i < tables.length; i++) {
    let thisTable = await getCityIO(tables[i]);

    let thisTableName = clearNames(tables[i]);

    infoDiv(
      i +
        " of " +
        tables.length +
        " CityScope tables: " +
        clearNames(tables[i]).link(tables[i])
    );

    let thisTableHeader = thisTable.header;
    let randPos = Math.random() * 10;
    let tableSpatial = thisTableHeader
      ? thisTableHeader.spatial
      : {
          latitude: 0,
          longitude: randPos
        };
    tableArray.push({
      url: tables[i],
      name: thisTableName,
      lat: tableSpatial.latitude,
      lon: tableSpatial.longitude
    });
  }

  makeMap(tableArray);
}
////////////////////////////////////////////////////////////////////////////////////

function makeMap(tablesArray) {
  var map = L.map("map").setView([51.505, -0.09], 1);
  // setup the map API
  L.tileLayer(
    "https://api.mapbox.com/styles/v1/relnox/cjg1ixe5s2ubp2rl3eqzjz2ud/tiles/512/{z}/{x}/{y}?access_token=pk.eyJ1IjoicmVsbm94IiwiYSI6ImNqd2VwOTNtYjExaHkzeXBzYm1xc3E3dzQifQ.X8r8nj4-baZXSsFgctQMsg",
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
    shadowAnchor: [0, -30]
  });

  for (var i = 0; i < tablesArray.length; i++) {
    //clear names of tables
    let url = tablesArray[i].url;
    url = clearNames(url);

    //create map marker
    let marker = new L.marker([tablesArray[i].lat, tablesArray[i].lon], {
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
      infoDiv("getting header for: " + url);
    });
  }

  // click event handler to creat a chart and show it in the popup
  async function modalSetup(m) {
    //get the divs for content
    var tableNameDiv = document.getElementById("tableNameDiv");
    //get the binded props
    let tableMeta = m.properties;
    tableNameDiv.innerHTML = clearNames(m.properties.url);

    var deleteDiv = document.getElementById("deleteDiv");
    deleteDiv.innerHTML = "<a href=" + delLink + ">Remove Table</a>";

    //put prj name in div
    let delLink =
      "https://cityio.media.mit.edu/api/table/clear/" + tableMeta.name;

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
  // only show table header
  let jsonMerge = {};
  $.extend(jsonMerge, cityIOjson.meta, cityIOjson.header);
  let cityIOjsonString = JSON.stringify(jsonMerge, null, 2);
  let tableHeaderDiv = document.getElementById("tableHeaderDiv");
  tableHeaderDiv.innerHTML = "";
  //
  output(syntaxHighlight(cityIOjsonString));
  //
  function output(inp) {
    tableHeaderDiv.appendChild(document.createElement("pre")).innerHTML = inp;
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
