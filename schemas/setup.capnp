@0xd519b37b1ff809a5;

using Rust = import "rust.capnp";
$Rust.parentModule("api");

interface Setup {
    setName @0 (newName :Text);
    setWifi @1 (ssid :Text, pass :Text);
}