fn main() {
    println!(
        "{}",
        serde_yaml::to_string(&rust_kata_004::TorHiddenService::crd()).unwrap()
    )
}
