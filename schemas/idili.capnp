@0xd8421f10e61a606f;

using Photos = import "photos.capnp";
using Setup = import "setup.capnp";
using Rust = import "rust.capnp";
$Rust.parentModule("api");

interface IdiliService {
    getPhotosService @0 () -> (photos :Photos.PhotoLibrary);
}