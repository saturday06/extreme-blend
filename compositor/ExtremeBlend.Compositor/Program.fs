module ExtremeBlend.Compositor

open ExtremeBlend.Compositor.IWaylandProtocol
open ExtremeBlend.Compositor.ObjectNotFound
open ExtremeBlend.Compositor.WlDisplay
open System
open System.Collections.Generic
open System.IO
open System.Net.Sockets
open System.Text
open System.Threading

let handleRequest (socket : Socket,
                   objects : Dictionary<uint32, IWaylandProtocol>,
                   senderObjectId : uint32, opcode : uint16, argsBuf : byte []) : unit =
    let object = objects.GetValueOrDefault(senderObjectId, ObjectNotFound())
    object.Invoke(socket, senderObjectId, opcode, argsBuf)
    ()

let readRequest (socket : Socket) : Option<uint32 * uint16 * byte []> =
    let senderObjectId =
        let buf : byte [] = Array.zeroCreate 4
        let bufRead = socket.Receive(buf)
        if buf.Length = bufRead then
            let senderObjectId = BitConverter.ToUInt32(buf, 0)
            Console.Write("senderObjectId=0x{0:X8}({0}) ", senderObjectId)
            Some(senderObjectId)
        else
            Console.WriteLine
                ("sender's object id read size is {0} but actually {1}",
                 buf.Length, bufRead)
            None

    let (messageSize, opcode) =
        let buf : byte [] = Array.zeroCreate 4
        let bufRead = socket.Receive(buf)
        if buf.Length = bufRead then
            let messageSizeAndOpcode = BitConverter.ToUInt32(buf, 0)
            let messageSize = uint16 (messageSizeAndOpcode >>> 16)
            let opcode = uint16 (messageSizeAndOpcode &&& uint32 0x0000ffff)
            Console.Write
                ("messageSize=0x{0:X4}({0}) opcode=0x{1:X4}({1}) ", messageSize,
                 opcode)
            (Some(messageSize), Some(opcode))
        else
            Console.WriteLine
                ("message size and opcode read size is {0} but actually {1}",
                 buf.Length, bufRead)
            (None, None)

    match senderObjectId, messageSize, opcode with
    | (Some(senderObjectId), Some(messageSize), Some(opcode)) ->
        if messageSize < uint16 8 then
            Console.Write("messageSize is lesser than 8")
            None
        else
            let buf : byte [] = Array.zeroCreate ((int messageSize) - 8)
            let bufRead = socket.Receive(buf)
            if bufRead > 0 then
                for i in 0..(bufRead - 1) do
                    Console.Write("{0:X2} ", buf.[i])
                Console.WriteLine(";")
            else Console.WriteLine("read zero bytes")
            if buf.Length = bufRead then Some(senderObjectId, opcode, buf)
            else
                Console.WriteLine
                    ("payload size is {0} but actually {1}", buf.Length, bufRead)
                None
    | _ -> None

let rec thread (socket : Socket, objects : Dictionary<uint32, IWaylandProtocol>) : unit =
    printfn "waiting for receive"
    match readRequest (socket) with
    | Some(senderObjectId, opcode, buf) ->
        handleRequest (socket, objects, senderObjectId, opcode, buf)
        thread (socket, objects)
    | None -> ()

[<EntryPoint>]
let main _argv =
    printfn "Hello World from F#!"
    let socketPath = "c:\\Temp\\temp.unixsock"
    File.Delete(socketPath)
    let endPoint = UnixDomainSocketEndPoint(socketPath)
    use serverSocket =
        new Socket(endPoint.AddressFamily, SocketType.Stream,
                   ProtocolType.Unspecified)
    serverSocket.Bind(endPoint)
    serverSocket.Listen(50000)
    while true do
        printfn "Waiting for a connection..."
        let clientSocket = serverSocket.Accept()
        let objects = Dictionary<uint32, IWaylandProtocol>()
        objects.Add(1u, WlDisplay())
        printfn "Accept"
        Thread(new ThreadStart(fun () -> thread (clientSocket, objects)))
            .Start()
        ()
    0
