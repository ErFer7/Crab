use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
struct PlatformsFile {
    platform: Vec<PlatformSpec>,
}

#[derive(Deserialize)]
struct PlatformSpec {
    name: String,
    feature: String,
    linker: String,
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let platforms_toml_path = manifest_dir.join("platforms.toml");

    println!("cargo:rerun-if-changed={}", platforms_toml_path.display());

    let platforms_toml = fs::read_to_string(&platforms_toml_path)
        .unwrap_or_else(|e| panic!("failed to read platforms.toml: {e}"));

    let platforms: PlatformsFile = toml::from_str(&platforms_toml)
        .unwrap_or_else(|e| panic!("failed to parse platforms.toml: {e}"));

    let all_platform_names: Vec<String> = platforms
        .platform
        .iter()
        .map(|b| format!("\"{}\"", b.name))
        .collect();
    println!(
        "cargo:rustc-check-cfg=cfg(platform, values({}))",
        all_platform_names.join(", ")
    );

    let feature_env_var = |feature: &str| -> String {
        format!("CARGO_FEATURE_{}", feature.to_uppercase().replace('-', "_"))
    };

    let enabled: Vec<&PlatformSpec> = platforms
        .platform
        .iter()
        .filter(|b| env::var(feature_env_var(&b.feature)).is_ok())
        .collect();

    let platform = match enabled.as_slice() {
        [] => panic!("No platform selected"),
        [single] => single,
        multiple => panic!(
            "Multiple platforms selected ({})",
            multiple
                .iter()
                .map(|b| b.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    };

    println!("cargo:rustc-cfg=platform=\"{}\"", platform.name);

    let linker_script = manifest_dir.join(&platform.linker);

    println!("cargo:rerun-if-changed={}", linker_script.display());
    println!("cargo:rustc-link-arg=-T{}", linker_script.display());
}
