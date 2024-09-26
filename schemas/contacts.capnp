@0xa41e9616a3b15d84;

using Rust = import "rust.capnp";
$Rust.parentModule("api");

struct Person {
    id @0 :UInt16;
    name @1 :Text;
    fullName @2 :Text;
    emails @3 :List(Text);
    addresses @4 :List(Text);
}