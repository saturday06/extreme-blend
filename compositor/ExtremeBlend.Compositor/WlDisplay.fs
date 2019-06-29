module ExtremeBlend.Compositor.WlDisplay

open System
open ExtremeBlend.Compositor.IWaylandProtocol

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
