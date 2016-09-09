using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using UnityEngine;
//using Newtonsoft.Json;

//[RequireComponent(typeof(TCPControllerJSON))]
public class TCPRequestHelperJSON: TCPControllerJSON
{
    private int LastDelta = 0;
    public void AddGetRandomCommentRequest(Action<CommentInfo,int> handler)
    {
        Action<Dictionary<string, object>> reading = (data) =>
        {
            if (data == null)
            {
                AddGetRandomCommentRequest(handler);
                return;
            }
            var id = (int)(long)data["id"];
            var text = (string)data["text"];
            var x = (float)(double)data["x"];
            var y = (float)(double)data["y"];
            var z = (float)(double)data["z"];
            var total = (int)(long)data["total"];
            var latlng = new LatLong(y, x);
            // Debug.LogFormat("Total number of comments is {0}", total);
            handler(new CommentInfo(id, text, latlng), total);
        };

        AddRequest("get_rnd_comment", null, reading);
    }

    /*public void AddCommentPostRequest(string text, List<KeyValuePair<int, int>> buildings, Vector3 pos)
    {
        var list = buildings.Select(b => new Dictionary<string, object>() { { "x", b.Key }, { "y", b.Value } }).ToList();
        var data = new Dictionary<string, object>()
        {
            { "text", text },
            { "x", pos.x },
            { "y", pos.y },
            { "z", pos.z },
            { "buildings", list }
        };
        controller.AddRequest("post_comment", data, (r) => { });
    }*/

    public void AddCommentPostRequest(string text, List<KeyValuePair<int, int>> buildings, LatLong geoloc)
    {
        var list = buildings.Select(b => new Dictionary<string, object>() { { "x", b.Key }, { "y", b.Value } }).ToList();
        var data = new Dictionary<string, object>()
        {
            { "text", text },
            { "x", geoloc.longitude },
            { "y", geoloc.latitude },
            { "z", 0 },
            { "buildings", list }
        };
        AddRequest("post_comment", data, (r) => { });
    }


    public void AddStatusRequest(Action<bool> handler)
    {
        AddRequest("get_status", null, (r) => { handler((bool)r["status"]); });
    }

    public void AddUpdateRequest(int width, int height, Action<GridInfo> handler)
    {
        Action<Dictionary<string, object>> reading = (reader) =>
        {
            var gridInfo = new GridInfo(width, height);

            var newDelta = (int)(long)reader["new_delta"];
            var grid = ((List<object>)reader["grid"]).ConvertAll(b => b as Dictionary<string, object>);
            if(grid.Count == 0)
            {
                AddUpdateRequest(width, height, handler);
                return;
            }
            foreach (var cell in grid)
            {
                
                var x = (int)(long)cell["x"];
                var y = (int)(long)cell["y"];
                var type = (int)(long)cell["type"];
                var rot = (int)(long)cell["rot"];
                var heat = (int)(long)cell["magnitude"];

                var cellData = new GridInfo.CellData(x, y, type, rot, heat);
                gridInfo.UpdateCell(cellData);
                Debug.Log(cellData);

            }
            var objects = (Dictionary<string, object>)reader["objects"];

            if (objects.ContainsKey("density"))
            {
                gridInfo.BuildingDensity = ((List<object>)objects["density"]).ConvertAll(b => (int)(long)b);
            }
            if (objects.ContainsKey("population"))
            {
                //gridInfo.PopulationInfo = 
                //gridInfo.PopulationInfo = ((List<object>)objects["population"]).ConvertAll(b => (int)(long)b);
            }
            //Debug.LogFormat("Density: {0}", string.Join(" ", gridInfo.BuildingDensity.Select(d => d.ToString()).ToArray()));
            //Debug.LogFormat("Population: {0}", string.Join(" ", gridInfo.PopulationInfo.Select(d => d.ToString()).ToArray()));

            LastDelta = newDelta;

            handler(gridInfo);
            
        };

        var data = new Dictionary<string, object>()
        {
            {"delta", LastDelta }
        };

        AddRequest("get_updates", data, reading);
    }

}
