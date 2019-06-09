module ExtremeBlend.Compositor

open System
open System.Collections.Generic
open System.IO
open System.Net
open System.Net.Sockets
open System.Text
open System.Threading

type IWaylandProtocol =
    abstract Invoke : Socket * uint32 * uint16 * byte [] -> unit

type ObjectNotFoundWaylandProtocol() =
    interface IWaylandProtocol with
        member this.Invoke(socket, senderObjectId, opcode, argsBuf) =
            let senderObjectIdBytes =
                BitConverter.GetBytes(senderObjectId : uint32)
            let codeBytes = BitConverter.GetBytes(0u) // invalid object
            let message = "invalid_object\x00"
            let messageBytes =
                Encoding.ASCII.GetBytes
                    (message.PadRight((message.Length + 3) / 4 * 4, '\x00'))
            let messageBytesLenBytes =
                BitConverter.GetBytes(messageBytes.Length : int32)
            let totalLen =
                8 + senderObjectIdBytes.Length + codeBytes.Length
                + messageBytesLenBytes.Length + messageBytes.Length
            Console.Write("len={0} ", totalLen)
            if totalLen > 0xffff then
                Console.Write
                    ("message len {0} is greater than 0xffff", totalLen)
            use memoryStream = new MemoryStream()
            memoryStream.Write(new ReadOnlySpan<byte>(senderObjectIdBytes))
            memoryStream.Write
                (new ReadOnlySpan<byte>(BitConverter.GetBytes
                                            ((uint32 totalLen <<< 16) ||| 0u)))
            memoryStream.Write(new ReadOnlySpan<byte>(senderObjectIdBytes))
            memoryStream.Write(new ReadOnlySpan<byte>(codeBytes))
            memoryStream.Write(new ReadOnlySpan<byte>(messageBytesLenBytes))
            memoryStream.Write(new ReadOnlySpan<byte>(messageBytes))
            socket.Send(memoryStream.ToArray()) |> ignore
            Console.WriteLine("unknown sender object")
            Console.WriteLine(";")

type WlDisplay() =
    interface IWaylandProtocol with
        member this.Invoke(socket, senderObjectId, opcode, argsBuf) =
            Console.Write("wl_display ")
            match opcode with
            | 0us ->
                Console.Write("sync ")
                if argsBuf.Length <> 4 then
                    Console.Write
                        ("args buffer size is not 4 but {0}", argsBuf.Length)
                else
                    let callback = BitConverter.ToUInt32(argsBuf, 0)
                    Console.Write("callback=0x{0:X4}({0}) ", callback)
            | 1us ->
                Console.Write("get_registry ")
                if argsBuf.Length <> 4 then
                    Console.Write
                        ("args buffer size is not 4 but {0}", argsBuf.Length)
                else
                    let registry = BitConverter.ToUInt32(argsBuf, 0)
                    Console.Write("registry=0x{0:X4}({0}) ", registry)
            | _ -> Console.Write("unknown opcode ")

let handleRequest (socket : Socket,
                   objects : Dictionary<uint32, IWaylandProtocol>,
                   senderObjectId : uint32, opcode : uint16, argsBuf : byte []) : unit =
    let object =
        objects.GetValueOrDefault
            (senderObjectId, ObjectNotFoundWaylandProtocol())
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
        objects.Add(1u, new WlDisplay())
        printfn "Accept"
        Thread(new ThreadStart(fun () -> thread (clientSocket, objects)))
            .Start()
        ()
    0
