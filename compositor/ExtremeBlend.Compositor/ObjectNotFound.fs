module ExtremeBlend.Compositor.ObjectNotFound

open ExtremeBlend.Compositor.IWaylandProtocol
open System
open System.IO
open System.Net.Sockets
open System.Text

type ObjectNotFound() =

    member this.invoke (socket : Socket, senderObjectId : uint32,
                        opcode : uint16, argsBuf : byte []) : unit =
        let senderObjectIdBytes = BitConverter.GetBytes(senderObjectId : uint32)
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
            Console.Write("message len {0} is greater than 0xffff", totalLen)
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

    interface IWaylandProtocol with
        member this.Invoke(socket, senderObjectId, opcode, argsBuf) =
            this.invoke (socket, senderObjectId, opcode, argsBuf)
