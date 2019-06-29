module ExtremeBlend.Compositor.IWaylandProtocol

open System.Net.Sockets

type IWaylandProtocol =
    abstract Invoke : Socket * uint32 * uint16 * byte [] -> unit
