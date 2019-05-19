open System
open System.Threading.Tasks
open Grpc.Core
open Reflector.Grpc.Proto

[<EntryPoint>]
let main argv =
    let channel = Channel("127.0.0.1:50051", ChannelCredentials.Insecure)
    printfn "Hello World from F#!"
    0 // return an integer exit code
