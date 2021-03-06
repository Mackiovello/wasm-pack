extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate wasm_pack;

mod utils;

use std::collections::HashSet;
use std::fs;

use wasm_pack::manifest;

#[test]
fn it_gets_the_crate_name_default_path() {
    assert!(manifest::get_crate_name(".").is_ok());
    assert_eq!(manifest::get_crate_name(".").unwrap(), "wasm-pack");
}

#[test]
fn it_gets_the_crate_name_provided_path() {
    assert!(manifest::get_crate_name("tests/fixtures/js-hello-world").is_ok());
    assert_eq!(
        manifest::get_crate_name("tests/fixtures/js-hello-world").unwrap(),
        "js-hello-world"
    );
}

#[test]
fn it_checks_has_cdylib_default_path() {
    let step = wasm_pack::progressbar::Step::new(1);
    assert!(manifest::check_crate_config(".", &step).is_err());
}

#[test]
fn it_checks_has_cdylib_provided_path() {
    let step = wasm_pack::progressbar::Step::new(1);
    assert!(manifest::check_crate_config("tests/fixtures/js-hello-world", &step).is_ok());
}

#[test]
fn it_checks_has_cdylib_wrong_crate_type() {
    let step = wasm_pack::progressbar::Step::new(1);
    assert!(manifest::check_crate_config("tests/fixtures/bad-cargo-toml", &step).is_err());
}

#[test]
fn it_creates_a_package_json_default_path() {
    let step = wasm_pack::progressbar::Step::new(1);
    let path = ".".to_string();
    wasm_pack::command::init::create_pkg_dir(&path, &step).unwrap();
    assert!(manifest::write_package_json(&path, &None, false, "", &step).is_ok());
    let package_json_path = format!("{}/pkg/package.json", &path);
    assert!(fs::metadata(package_json_path).is_ok());
    assert!(utils::read_package_json(&path).is_ok());
    let pkg = utils::read_package_json(&path).unwrap();
    assert_eq!(pkg.name, "wasm-pack");
    assert_eq!(pkg.repository.ty, "git");
    assert_eq!(
        pkg.repository.url,
        "https://github.com/ashleygwilliams/wasm-pack.git"
    );
    assert_eq!(pkg.main, "wasm_pack.js");
    let types = pkg.types.unwrap_or_default();
    assert_eq!(types, "wasm_pack.d.ts");

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> = ["wasm_pack_bg.wasm", "wasm_pack.d.ts"]
        .iter()
        .map(|&s| String::from(s))
        .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_creates_a_package_json_provided_path() {
    let step = wasm_pack::progressbar::Step::new(1);
    let path = "tests/fixtures/js-hello-world".to_string();
    wasm_pack::command::init::create_pkg_dir(&path, &step).unwrap();
    assert!(manifest::write_package_json(&path, &None, false, "", &step).is_ok());
    let package_json_path = format!("{}/pkg/package.json", &path);
    assert!(fs::metadata(package_json_path).is_ok());
    assert!(utils::read_package_json(&path).is_ok());
    let pkg = utils::read_package_json(&path).unwrap();
    assert_eq!(pkg.name, "js-hello-world");
    assert_eq!(pkg.main, "js_hello_world.js");

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> = ["js_hello_world_bg.wasm", "js_hello_world.d.ts"]
        .iter()
        .map(|&s| String::from(s))
        .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_creates_a_package_json_provided_path_with_scope() {
    let step = wasm_pack::progressbar::Step::new(1);
    let path = "tests/fixtures/scopes".to_string();
    wasm_pack::command::init::create_pkg_dir(&path, &step).unwrap();
    assert!(
        manifest::write_package_json(&path, &Some("test".to_string()), false, "", &step).is_ok()
    );
    let package_json_path = format!("{}/pkg/package.json", &path);
    assert!(fs::metadata(package_json_path).is_ok());
    assert!(utils::read_package_json(&path).is_ok());
    let pkg = utils::read_package_json(&path).unwrap();
    assert_eq!(pkg.name, "@test/scopes-hello-world");
    assert_eq!(pkg.main, "scopes_hello_world.js");

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> = ["scopes_hello_world_bg.wasm", "scopes_hello_world.d.ts"]
        .iter()
        .map(|&s| String::from(s))
        .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_creates_a_pkg_json_with_correct_files_on_node() {
    let step = wasm_pack::progressbar::Step::new(1);
    let path = ".".to_string();
    wasm_pack::command::init::create_pkg_dir(&path, &step).unwrap();
    assert!(manifest::write_package_json(&path, &None, false, "nodejs", &step).is_ok());
    let package_json_path = format!("{}/pkg/package.json", &path);
    assert!(fs::metadata(package_json_path).is_ok());
    assert!(utils::read_package_json(&path).is_ok());
    let pkg = utils::read_package_json(&path).unwrap();
    assert_eq!(pkg.name, "wasm-pack");
    assert_eq!(pkg.repository.ty, "git");
    assert_eq!(
        pkg.repository.url,
        "https://github.com/ashleygwilliams/wasm-pack.git"
    );
    assert_eq!(pkg.main, "wasm_pack.js");
    let types = pkg.types.unwrap_or_default();
    assert_eq!(types, "wasm_pack.d.ts");

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> =
        ["wasm_pack_bg.wasm", "wasm_pack_bg.js", "wasm_pack.d.ts"]
            .iter()
            .map(|&s| String::from(s))
            .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_creates_a_package_json_with_correct_keys_when_types_are_skipped() {
    let step = wasm_pack::progressbar::Step::new(1);
    let path = ".".to_string();
    wasm_pack::command::init::create_pkg_dir(&path, &step).unwrap();
    assert!(manifest::write_package_json(&path, &None, true, "", &step).is_ok());
    let package_json_path = format!("{}/pkg/package.json", &path);
    assert!(fs::metadata(package_json_path).is_ok());
    assert!(utils::read_package_json(&path).is_ok());
    let pkg = utils::read_package_json(&path).unwrap();
    assert_eq!(pkg.name, "wasm-pack");
    assert_eq!(pkg.repository.ty, "git");
    assert_eq!(
        pkg.repository.url,
        "https://github.com/ashleygwilliams/wasm-pack.git"
    );
    assert_eq!(pkg.main, "wasm_pack.js");

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> = ["wasm_pack_bg.wasm"]
        .iter()
        .map(|&s| String::from(s))
        .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_errors_when_wasm_bindgen_is_not_declared() {
    let step = wasm_pack::progressbar::Step::new(1);
    assert!(manifest::check_crate_config("tests/fixtures/bad-cargo-toml", &step).is_err());
}

#[test]
fn it_does_not_error_when_wasm_bindgen_is_declared() {
    let step = wasm_pack::progressbar::Step::new(1);
    assert!(manifest::check_crate_config("tests/fixtures/js-hello-world", &step).is_ok());
}
