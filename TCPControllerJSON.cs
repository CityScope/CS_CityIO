using UnityEngine;
using System.Collections;
using System.IO;
using System;
using System.Linq;
using System.Text;
using System.Collections.Generic;
using System.Threading;
using MiniJSON;
//using CallbackInfo = TCPControllerJSON.TCallbackInfo<System.Collections.Generic.Dictionary<string, object>>;

public class TCPControllerJSON : Singleton<TCPControllerJSON>
{
    

    public class BaseJSONPacket : Dictionary<string, object>
    {
        public int id { get { return (int)(long)this["id"]; } }
        public string opcode { get { return (string)this["opcode"]; } }
        public Dictionary<string, object> data { get { return (Dictionary<string, object>)this["data"]; } }

        public BaseJSONPacket(string operation, Dictionary<string, object> data) : base()
        {
            Add("id", (long)UnityEngine.Random.Range(0, int.MaxValue));
            Add("opcode", operation);
            Add("data", data);
        }

        public BaseJSONPacket(Dictionary<string, object> dict) : base(dict) { }
    }

    public struct CallbackInfo<T>
    {
        public Action<T> Action;
        public T Data;

        public CallbackInfo(Action<T> action, T data)
        {
            Action = action;
            Data = data;
        }
    }
    
    /*private struct CallbackInfo: TCallbackInfo<Dictionary<string, object>>
    {

    }*/

    private class PacketListWrapper
    {
        public List<BaseJSONPacket> packets;

        public PacketListWrapper(List<BaseJSONPacket> list)
        {
            this.packets = list;
        }
    }

    private class PacketInfo
    {
        public BaseJSONPacket requestData;
        public Action<Dictionary<string, object>> handler;
        public int id;

        public PacketInfo(BaseJSONPacket request, Action<Dictionary<string, object>> handler)
        {
            this.id = request.id;
            this.requestData = request;
            this.handler = handler;
        }
    }


    //variables

    public TCPConnection myTCP;

    public string Host = "localhost";
    public int Port = 9999;
    public string tableName;

    private Thread receiveThread;
    private object inputLock = new object();
    private object outputLock = new object();

    private List<PacketInfo> pendingRequests;
    private Dictionary<int, PacketInfo> expectedRequests;

    private List<CallbackInfo<Dictionary<string, object>>> callbackQueue;
    
    void Awake()
    {
        myTCP = new TCPConnection(Host, Port);

        callbackQueue = new List<CallbackInfo<Dictionary<string, object>>>(); //new Dictionary<Action<Dictionary<string, object>>, Dictionary<string, object>>();

        pendingRequests = new List<PacketInfo>();
        expectedRequests = new Dictionary<int, PacketInfo>();


        receiveThread = new Thread(SocketThread);
        receiveThread.IsBackground = true;
        receiveThread.Start();
    }

    void Update()
    {
        lock (outputLock)
        {
            foreach (var callback in callbackQueue)
            {
                callback.Action(callback.Data);
            }
            callbackQueue.Clear();
        }
    }

    void OnApplicationQuit()
    {
        receiveThread.Abort();
        myTCP.CloseSocket();
    }

    void SocketThread()
    {

        SetupConnection();

        while (true)
        {
            Thread.Sleep(500);
            if (!myTCP.IsReady)
            {
                Debug.Log("Trying to re-connect");
                SetupConnection();
            }
            if (myTCP.IsReady)
            {
                try
                {
                    //if (SocketResponse())
                    SocketQuery();
                    SocketResponse();
                }
                catch (Exception e)
                {
                    Debug.LogException(e);
                }
            }
        }
    }

    void SetupConnection()
    {
        //try to connect
        Debug.Log("Attempting to connect..");
        myTCP.SetupSocket();
        if (myTCP.IsReady)
        {
            Debug.Log("Attempting to login");
            var dict = new Dictionary<string, string>() {
                {"salt", "CIOMIT" },
                {"type", "client" },
                {"table", tableName }
            };

            myTCP.WriteString(Json.Serialize(dict));

            int count = 0;
            byte[] response;
            while (count < 3)
            {
                count += myTCP.ReadSocket(out response);
            }

            //pendingRequests = new List<PacketInfo>();
            pendingRequests.AddRange(expectedRequests.Select(r => r.Value).ToList());
            expectedRequests.Clear();
            //pendingRequests.ForEach(p => Debug.Log(p.requestData.opcode));
        }
        else
        {
            Debug.LogError("Socket is not ready");
        }
    }

    private void SocketQuery()
    {
        //Building JSON list with the pending requests
        //var elements = pendingRequests.Select(r => JsonUtility.ToJson(r.requestData)).ToList();
        //var requestString = "{[" + String.Join(",", elements) + "]}";
        string requestString;
        lock (inputLock)
        {
            if (pendingRequests.Count == 0)
                return;

            var elements = pendingRequests.Select(r => r.requestData).ToList();
            //var requestString = Json.Serialize(new Dictionary<string, object>() { { "packets", elements } });
            requestString = Json.Serialize(elements);

            //Translate the pending requests into expected requests
            expectedRequests = pendingRequests.ToDictionary(r => r.id, r => r);
            pendingRequests.Clear();
        }
        myTCP.WriteString(requestString);


    }

    private bool SocketResponse()
    {
        if (expectedRequests.Count == 0)
            return true;

        byte[] msg;
        List<System.Object> results = null;
        string resultString = "";
        int count = 0;
        while (results == null && count < 3000)
        {
            var len = myTCP.ReadSocket(out msg);
            if (len > 0)
            {
                resultString += System.Text.Encoding.UTF8.GetString(msg, 0, len);
                try
                {
                    results = (Json.Deserialize(resultString) as List<System.Object>);//.ConvertAll(b => b as BaseJSONPacket);
                }
                catch (Exception)
                {
                    Debug.LogWarning("Error while JSON parsing.");
                    //Debug.LogException(e);
                    Debug.LogWarningFormat("String: {0}", resultString);
                    //Debug.LogErrorFormat("Byte size vs real size: {0} vs {1}", len, resultString.Length);
                    // Debug.LogErrorFormat("Byte size vs real size: {0} vs {1}", len, resultString.Length);
                }
                if (results == null)
                {
                    Debug.Log(len);
                    //string hex = BitConverter.ToString(msg);
                    //Debug.LogFormat(hex);
                    Debug.Log(resultString);
                }

            }
            if (results == null)
            {
                Thread.Sleep(50);
            }

            count += 50;
        }
        if (results == null || resultString.Length == 0)
        {
            Debug.Log(resultString);
            Debug.LogFormat("resStr len = {0}", resultString.Length);
        }
        if (results == null)
        {
            Debug.LogWarning("Connection lost. Trying to reconnect");
            SetupConnection();
            return false;
        }
        foreach (var obj in results)
        {
            var r = new BaseJSONPacket(obj as Dictionary<string, object>);
            var id = r.id;
            //Debug.LogFormat("Respone to {0}", expectedRequests[r.id].requestData.opcode);
            lock (outputLock)
            {
                //callbackQueue[expectedRequests[id].handler] = r.data;
                var callback = new CallbackInfo<Dictionary<string, object>>(expectedRequests[id].handler, r.data);
                callbackQueue.Add(callback);
            }
            expectedRequests.Remove(id);
        }
        return true;

    }


    public void AddRequest(string opcode, Dictionary<string, object> requestData, Action<Dictionary<string, object>> handler)
    {
        lock (inputLock)
        {
            var packet = new PacketInfo(new BaseJSONPacket(opcode, requestData), handler);
            // Debug.LogFormat("Added request[{1}] to {0}", opcode, packet.id);
            pendingRequests.Add(packet);
        }
    }

}