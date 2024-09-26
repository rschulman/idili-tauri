@0xbcb7256585409cfc;

using Person = import "contacts.capnp";
using Rust = import "rust.capnp";
$Rust.parentModule("api");

struct DateTime {
    date @0 :Date;
    time @1 :Time;
}

struct Date {
    year @0 :Int32;
    month @1 :UInt32;
    day @2 :UInt32;
}

struct Time {
    hour @0 :UInt32;
    minute @1 :UInt32;
    second @2 :UInt32;
}

struct Photo {
    id @0 :Int32;
    lat @1 :Float32;
    long @2 :Float32;
    width @3 :Int32;
    height @4 :Int32;
    date @5 :DateTime;
    # subjects @6 :List(Person.Person);
}

interface PhotoLibrary {
    list @0 () -> (list :List(Int32));
    getMetadata @1 (id :Int32) -> (photo :Photo);
    get @2 (id :Int32) -> (bytes :Data);
    upload @3 (bytes :Data) -> (id :Int32);
    search @4 (tags :Text) -> (list :List(Int32));
}