fn main() {
    prost_build::compile_protos(&["src/beats.proto"],
                                &["src/"]).unwrap();
}
