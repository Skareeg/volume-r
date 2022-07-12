# volume-r
Rust crate based on OpenVDB.

Replication of OpenVDB is not gonna fully happen in a performant way just due to the abhorrent use of template metaprogramming in OpenVDB. (The tree structure is built at compile time via metaprogramming. Thanks C++.)

I could do the same with Rust, but this is gonna take some time. This crate will be developed alongside Arcana (A procedural VR sculpting and layout toolset) and ProcFlow (a node graph based procedural data flow system that will form the core of Arcana. Think Houdini.)

This will need to be *very* performant, as it will be used in real time settings. I may make a piece of this link via the [ash] crate to develop out GPU side compute operations that will play nice with a render graph where necessary, but I will keep this particular crate modular and as dependency free as possible. (Using a feature or something.)
