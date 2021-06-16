
extern crate protoc_rust;
use protoc_rust::{ Codegen, Customize };

fn main() {
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

