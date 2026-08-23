#[test]
fn dory_bin_is_built() {
    let bin = env!("CARGO_BIN_EXE_dory");
    assert!(std::path::Path::new(bin).is_file(), "missing {bin}");
}
