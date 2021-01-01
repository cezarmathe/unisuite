//! Build script that automatically compiles protos.

const PROTOS: &[&str] = &["com/cezarmathe/unisuite/mevents.proto"];

fn main() {
    tonic_build::configure().compile(PROTOS, &["./protos"]).unwrap();
}
