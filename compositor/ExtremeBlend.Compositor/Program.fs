module ExtremeBlend.Compositor

open System
open System.IO
open System.Net
open System.Net.Sockets
open System.Threading

let handleRequest (socket : Socket, senderObjectId : uint32, opcode : uint16,
                   argsBuf : byte []) : unit =
    match senderObjectId with
    | 1u ->
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
    | _ -> Console.WriteLine("unknown sender object")
    Console.WriteLine(";")
    ()

let rec thread (socket : Socket) : unit =
    printfn "waiting for receive"
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
            ()
        else
            let buf : byte [] = Array.zeroCreate ((int messageSize) - 8)
            let bufRead = socket.Receive(buf)
            if bufRead > 0 then
                for i in 0..(bufRead - 1) do
                    Console.Write("{0:X2} ", buf.[i])
                Console.WriteLine(";")
            else Console.WriteLine("read zero bytes")
            if buf.Length = bufRead then
                handleRequest (socket, senderObjectId, opcode, buf)
                thread (socket)
            else
                Console.WriteLine
                    ("payload size is {0} but actually {1}", buf.Length, bufRead)
                ()
    | _ -> ()

[<EntryPoint>]
let main argv =
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
        printfn "Accept"
        Thread(new ThreadStart(fun () -> thread (clientSocket))).Start()
        ()
    0
