fn main() {
    println!("cargo:rerun-if-changed=info.json");
    let v: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string("info.json").unwrap()).unwrap();
    println!("cargo:rustc-env=version={}", v["ver"].as_str().unwrap());
}
