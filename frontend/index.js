import "babel-polyfill";
import * as L from "leaflet";
window.$ = require("jquery");
import "bootstrap";
import * as legoIO from "/img/legoio.png";
import * as shadow from "/img/shadow.png";

////////////////////////////////////////////////////////////////////////////////////

var updateInterval = 2000;

/**
 * get cityIO method [uses polyfill]
 * @param cityIOtableURL cityIO API endpoint URL
 */
async function getCityIO(url, myHeaders) {
  var myHeaders = {
    headers: new Headers({
      Authorization:
        "Bearer 86c1e6d8f574a51896bf02e8622b858d573b8afd4e583d3b9258cfe8ed336ee7"
    })
  };

  return fetch(url, myHeaders)
    .then(function(response) {
      return response.json();
    })
    .then(function(cityIOdata) {
      return cityIOdata;
    })
    .catch(err => {
      console.log(err);
    });
}

////////////////////////////////////////////////////////////////////////////////////
function clearNames(url) {
  return url.toString().replace("https://cityio.media.mit.edu/api/table/", "");
}
////////////////////////////////////////////////////////////////////////////////////

async function getTables() {
  let counter = 0;
  let tableArray = [];
  let cityIOurl = "https://cityio.media.mit.edu/api/tables/list";

  const tables = await getCityIO(cityIOurl);

  for (let i = 0; i < tables.length; i++) {
    let thisTable = await getCityIO(tables[i]);

    // make sure we can actually GET the table now
    if (thisTable) {
      let thisTableName = clearNames(tables[i]);
      infoDiv(
        i +
          " of " +
          tables.length +
          " CityScope tables: " +
          clearNames(tables[i]).link(tables[i])
      );

      let thisTableHeader = thisTable.header;
      let tableSpatial;
      if (thisTableHeader && thisTableHeader.spatial) {
        tableSpatial = thisTableHeader.spatial;
      } else {
        counter = counter + 1;

        tableSpatial = { latitude: counter, longitude: counter };
      }

      tableArray.push({
        url: tables[i],
        name: thisTableName,
        lat: tableSpatial.latitude,
        lon: tableSpatial.longitude
      });
    }
  }

  makeMap(tableArray, counter);
}
////////////////////////////////////////////////////////////////////////////////////

function getDistance(destination) {
  // return distance in meters
  var lon1 = toRadian(0),
    lat1 = toRadian(0),
    lon2 = toRadian(destination[1]),
    lat2 = toRadian(destination[0]);

  var deltaLat = lat2 - lat1;
  var deltaLon = lon2 - lon1;

  var a =
    Math.pow(Math.sin(deltaLat / 2), 2) +
    Math.cos(lat1) * Math.cos(lat2) * Math.pow(Math.sin(deltaLon / 2), 2);
  var c = 2 * Math.asin(Math.sqrt(a));
  var EARTH_RADIUS = 6371;
  return c * EARTH_RADIUS * 1000;
}
function toRadian(degree) {
  return (degree * Math.PI) / 180;
}

////////////////////////////////////////////////////////////////////////////////////

function makeMap(tablesArray, counter) {
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

  let circle = L.circle([0, 0], getDistance([counter, counter]), {
    color: "#ed5066",
    fill: false,
    opacity: 0.5
  }).addTo(map);

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
    //put prj name in div
    let delLink =
      "https://cityio.media.mit.edu/api/table/clear/" + tableMeta.name;

    deleteDiv.innerHTML = "<a href=" + delLink + ">Remove Table</a>";

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

// getCityIO("https://cityio.media.mit.edu/api/table/hidden_table", myHeaders);
