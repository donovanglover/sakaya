use assert_cmd::Command;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    package: Option<PackageConfig>,
}

#[derive(Debug, Deserialize)]
struct PackageConfig {
    version: Option<String>,
    authors: Option<Vec<String>>,
    description: Option<String>,
}

#[test]
/// Ensures that the copyright year is updated in both files if the LICENSE is updated
fn copyright_is_the_same() {
    let license = &fs::read_to_string("LICENSE").unwrap();
    let license = license.split("\n").collect::<Vec<&str>>()[0];

    let cargo = &fs::read_to_string("Cargo.toml").unwrap();
    let cargo: Config = toml::from_str(cargo).unwrap();
    let cargo = &cargo.package.unwrap().authors.unwrap()[0];

    assert!(
        cargo.starts_with(license),
        "Cargo.toml should have the same copyright year as LICENSE"
    );
}

#[test]
/// Ensures that the usage code block in the README is the same as the output of sakaya -h
fn usage_is_the_same() {
    let cargo_description = &fs::read_to_string("Cargo.toml").unwrap();
    let cargo_description: Config = toml::from_str(cargo_description).unwrap();
    let cargo_description = cargo_description.package.unwrap().description.unwrap();

    let readme = &fs::read_to_string("README.md").unwrap();
    let mut inside_code_block = false;

    // Initialize with cargo_description since we don't duplicate this in the README
    let mut readme_usage: String = format!("{cargo_description}\n\n");

    for line in readme.lines() {
        if line == "```" {
            inside_code_block = false;
            continue;
        }

        if inside_code_block {
            readme_usage.push_str(&(line.to_owned() + "\n"));
            continue;
        }

        if line == "```man" {
            inside_code_block = true;
        }
    }

    let mut cmd = Command::cargo_bin("sakaya").unwrap();
    cmd.arg("-h").assert().stdout(readme_usage);
}

#[test]
/// Ensures that the correct version of sakaya is found in the README
fn current_version_is_used() {
    let cargo_version = &fs::read_to_string("Cargo.toml").unwrap();
    let cargo_version: Config = toml::from_str(cargo_version).unwrap();
    let cargo_version = cargo_version.package.unwrap().version.unwrap();

    let readme = &fs::read_to_string("README.md").unwrap();

    assert!(
        readme.contains(&("--tag ".to_owned() + cargo_version.as_str())),
        "should have the correct tag version in the README"
    )
}
