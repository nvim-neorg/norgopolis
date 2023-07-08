# Norgopolis

([Client Implementation](https://github.com/nvim-neorg/norgopolis-client) / [Module Implementation](https://github.com/nvim-neorg/norgopolis-module))

Norgopolis is a lightweight communication, launcher and utility services client for the Neorg rust-native modules ecosystem on Desktop. It is designed to connect, launch and manage instances of modules such as the [Norgberg database system](https://github.com/SevorisDoe/Norgberg) which should only exist once per desktop, but provide services to *n* frontends such as multiple neovim processes, without the need for intermediate process ownership changes or process re-launches.

Norgopolis also provides for the ability to download and connect different services together without the need for recompilation or complete re-launch, making for easy modifying and extension by the common user.

## Why?

There are four problems that Norgopolis solves:

- how do we provide singleton services like a database-over-the-notes-on-file while having more than one frontend at the same time?
  
- How do we ensure that the exit of the first-launching frontend (which must initiate all the background services) does not cause loss of service, interruption of service, failure to handover to another thread, or similar issues?
  
  - how do we ensure that the processes exit when the last frontend has indeed also exited?
    
- How do we allow people to extend their installation with new native modules without the need for recompilation or volatile shared object dynamic loading?
  
- How do we get these modules to talk with each other and any frontend callees in a lightweight, performant manner?
  

Norgopolis addresses these.

## How it works

Norgopolis comes on two parts, the client and the server backend. They communicate with each other using gRPC.

When a client is launched, it tries to find an existing server instant via gRPC. If it does not find an existing instance, it will initiate a server boot as an independent process. If it finds an existing server, it will connect to that server and log on as a client session.

The Norgopolis server reference-counts registered clients. If clients die or time out with no communication for a certain duration, they are removed from the reference count. If the reference count hits zero, Norgopolis will gracefully exit itself and all its child processes.

The core of Norgopolis is a communications router between the frontend gRPC connection(s) and the set of currently loaded modules. "Modules" are executable binaries that conform to the Neorg native module standard, providing a configurations TOML and a stdin/stdout-based RPC communications interface. Modules provide services to their callers. Norgberg is an example of such a module, providing database services.

Related to the router is Norgopolis' loader capability. Norgopolis routes calls using a module name specified in the RPC protocol buffer data. If no such module is currently registered on the router, Norgopolis will attempt to locate the binary in a modules directory and spawn a child thread with that binary in it. This allows for inherent lazy loading, "self-loading" and adding-modules-underway. Modules may declare transient module dependencies in their configurations TOML, in which case the Norgobolis loader will verify availability and fail the load if dependencies are not availiable. Likewise, inabililty to locate an executable binary will fail gracefully.

Modules only have to be compiled binaries or otherwise executable programs. They run as independent child threads of Norgopolis. Note: that means of Norgopolis exists, so will these child threads. This is by design and should be bypassed sparingly. Norgopolis will broadcast shutdown RPC messages when it exits.

This also describes the utilities Norgopolis offers: they are related to loading, managing and shutting down modules.

### Client interface

The client exposes Rust functions and an FFI interface aimed at Lua, and comes both as a separate crate and as part of the combined crate. This is used to bridge method calls over into gRPC and execute the stateful startup-registration logic for the server, as well as unpack and deliver method invocation return values.

### Server interface

The server talks with any clients or other external callers which are not modules it maintains over gRPC, using the Norgopolis protobuffer spec. Note, this does mean you can try to call the server (if it is online) and through it any modules you may want, from any other application with the correct protocol buffer and mpack specification, such as web applet plugins or one-off scripts.

Modules talk to the server and through it with callers and other modules using stdin/stdout-piped protobuffers that follow the same specification. The server provides the asynchronous routing.

## Neat hacks and implications

Since Norgopolis launches executable files as child threads and communicates using protocol buffers over stdin/stdout, any executable that can talk according to our spec can be loaded as a module. This not only inoculates against Rust ABI changes between versions, but also allows modules compiled in other languages (or interpreted on the spot) to be loaded up into your system. Be however aware of the performance implications (modules run as separate threads, so slow programs will begin clogging your CPU), and modules are generally understood to be persistent, not transient, until unloaded with a command to the server or the server exits, and thus also all module threads. (That said, you can exit as a module yourself. Again, just beware of the implications.)

### Self-loading

Since Norgopolis will attempt to locate and launch modules for which it has received a method call but does not have an active instance to route to, you can use this to inject an executable into the modules group from external gRPC calls, by having an executable containing Neorg module logic first call the server, provide its loading spec, exist its initial thread and then launch as a module thread under the server. This can be a very neat method to run modules one-off as part of a script ecosystem, or for cases such as testing.

### Default lazy loading

Norgopolis default is to only try and load a module if a call is received and it does not have an active thread of that module name to route to. This implicit lazy loading can be used to reduce startup times. Norgopolis has a loading capability to automate the startup of certain services in part due to its transient dependency checking, but this builds on the lazy loading behavior.
