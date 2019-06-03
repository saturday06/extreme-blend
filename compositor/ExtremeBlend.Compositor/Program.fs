module ExtremeBlend.Compositor

open System
open System.IO
open System.Net
open System.Net.Sockets
open System.Threading

let rec thread (socket : Socket) =
    printfn "Waiting for receive"
    let buf : byte [] = Array.zeroCreate (4096 * 16)
    let len = socket.Receive(buf)
    if len > 0 then
        Console.WriteLine("Received {0} bytes", len)
        for i in 0..(len - 1) do
            Console.Write("{0:X2} ", buf.[i])
        Console.WriteLine(";")
        thread (socket)

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
