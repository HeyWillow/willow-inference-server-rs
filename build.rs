use cargo_license::{GetDependenciesOpt, get_dependencies_from_cargo_lock, write_json};
use cargo_metadata::MetadataCommand;
use serde_json::Value;

fn main() {
    #[cfg(all(feature = "hf", not(feature = "stt")))]
    compile_error!("feature `hf` is only used by `stt`, aborting build");

    built::write_built_file().expect("failed to gather build-time info");

    let mut cmd_metadata = MetadataCommand::new();
    cmd_metadata.other_options(vec![
        "--filter-platform".to_string(),
        std::env::var("TARGET").expect("TARGET environment variable missing"),
    ]);
    let dep_opts = GetDependenciesOpt {
        avoid_build_deps: true,
        avoid_dev_deps: true,
        ..Default::default()
    };
    let deps = get_dependencies_from_cargo_lock(&cmd_metadata, &dep_opts)
        .expect("failed to get dependencies from Cargo.lock");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR environment variable missing");
    // let path = std::path::Path::new(&out_dir).join("licenses.json");
    let path = format!("{out_dir}/licenses.json");
    let file = std::fs::File::create(&path).expect("failed to create file {path}");
    let mut writer: Box<dyn std::io::Write> = Box::new(file);
    write_json(&deps, &mut writer).expect("failed to write licenses.json");

    let licenses = std::fs::read_to_string(&path).expect("failed to read {path}");

    if let Err(e) = serde_json::from_str::<Value>(&licenses) {
        panic!("failed to deserialize licenses.json: {e}");
    }

    println!("cargo:rerun-if-changed=.git/HEAD");
}
