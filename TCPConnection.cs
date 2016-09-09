using UnityEngine;
using System.Collections;
using System;
using System.IO;
using System.Net.Sockets;

public class TCPConnection
{
    //ip/address of the server, 127.0.0.1 is for your own computer
    private string Host;

    //port for the server
    private int Port;

    //a true/false variable for connection status
    public bool IsReady { get { return Socket != null && Socket.Connected; } }

    private TcpClient Socket;
    public NetworkStream Stream
    {
        get; private set;
    }
    private BinaryWriter Writer;
    private BinaryReader Reader;


    public TCPConnection(string host, int port)
    {
        //IsReady = false;
        Host = host;
        Port = port;
    }


    //try to initiate connection
    public void SetupSocket()
    {
        try
        {
            Socket = new TcpClient(Host, Port);
            Stream = Socket.GetStream();
            Writer = new BinaryWriter(Stream);
            Reader = new BinaryReader(Stream);
            //IsReady = true;
        }
        catch (Exception e)
        {
            Socket = null;
            Debug.Log("Socket error:" + e);
        }
    }

    public void WriteString(string str)
    {
        if (!IsReady)
            return;


        Writer.Write(str.ToCharArray());
        Writer.Flush();
    }

    //send message to server
    public void WriteSocket(byte[] bytes)
    {
        if (!IsReady)
            return;
            
        
        Writer.Write(bytes);
        Writer.Flush();
    }

    //read message from server
    public int ReadSocket(out byte[] inStream)
    {
        if (!IsReady)
        {
            inStream = null;
            return 0;
        }

        int len = 0;
        inStream = new byte[Socket.SendBufferSize];
        if (Stream.DataAvailable)
        {
            len = Stream.Read(inStream, 0, inStream.Length);
        }
        return len;
    }

    //disconnect from the socket
    public void CloseSocket()
    {
        if (!IsReady)
            return;
        Writer.Close();
        Reader.Close();
        Socket.Close();
        //IsReady = false;
    }

    //keep connection alive, reconnect if connection lost
    public void MaintainConnection()
    {
        if (IsReady && !Stream.CanRead)
        {
            //setupSocket();
            //IsReady = false;
            Debug.LogError("Socket Failed");
        }
    }


}