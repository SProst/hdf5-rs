#[macro_use]
extern crate hdf5_derive;

#[derive(H5Type)]
//~^ ERROR proc-macro derive
//~^^ HELP Cannot derive H5Type for empty enums
enum Foo {}
