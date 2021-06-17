
#[cfg(all(feature = "protoc", not(feature = "pure")))]
use protoc_rust::{ Codegen, Customize };

#[cfg(all(feature = "pure", not(feature = "protoc")))]
use protobuf_codegen_pure::{ Codegen, Customize };

fn main() {
    if cfg!(feature = "pure") == cfg!(feature = "protoc") {
        panic!("You must enable either the protoc or the pure feature but not both.");
    }

    if cfg!(feature = "pure") {
        panic!("Unfortunately, protobuf-codegen-pure does not properly generate code for CS:GO proto files (yet?).");
    }

    Codegen::new()
        .out_dir("src/protos")
        .inputs(&[
            "protos/csgo/netmessages.proto",
            "protos/csgo/cstrike15_usermessages.proto",
            "protos/csgo/cstrike15_gcmessages.proto",
            "protos/csgo/steammessages.proto",
            "protos/csgo/engine_gcmessages.proto"
        ])
        .include("protos/csgo")
        .customize(Customize {
            expose_oneof: Some(true),
            expose_fields: Some(true),
            generate_accessors: Some(false),
            serde_derive: Some(true),
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .run()
        .expect("Unable to generate rust code for protobuf files.");
}

