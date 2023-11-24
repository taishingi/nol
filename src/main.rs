use eywa::{mkdir, shell, touch_with_content};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::File,
    path::Path,
    process::{exit, ExitCode},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    repositories: Vec<String>,
    dependencies: Vec<String>,
    quiet: bool,
}

fn create_config_file() {
    let config: String = format!("{}/.config/nol/nol.yml", env!("HOME"));
    if !Path::new(config.as_str()).is_file() {
        touch_with_content!(
            config.as_str(),
            "quiet: false\nrepositories:\n  - nol\ndependencies:\n  - testing_unit\n"
        );
    }
}

fn create_config_dir() {
    let config_dir: String = format!("{}/.config/nol", env!("HOME"));

    let nol_dir: String = format!("{}/Nol", env!("HOME"));

    if !Path::new(config_dir.as_str()).is_dir() {
        mkdir!(config_dir.as_str());
    }

    if !Path::new(nol_dir.as_str()).is_dir() {
        mkdir!(nol_dir.as_str());
    }
}

fn parse_config() -> Config {
    let f: File = File::open(format!("{}/.config/nol/nol.yml", env!("HOME")).as_str())
        .expect("failed to open file");
    serde_yaml::from_reader(f).expect("Failed to parse value")
}

fn main() -> ExitCode {
    create_config_dir();
    create_config_file();
    let nol: Config = parse_config();
    if !Path::new(format!("{}/Nol", env!("HOME")).as_str()).is_dir() {
        mkdir!(format!("{}/Nol", env!("HOME")).as_str());
    }

    for repository in nol.repositories.iter() {
        let dir: String = format!("{}/Nol/{}", env!("HOME"), repository);
        if nol.quiet {
            if !Path::new(dir.as_str()).is_dir() {
                assert!(shell!(
                    format!("{}/Nol", env!("HOME")).as_str(),
                    "cargo",
                    vec!["new", "--bin", repository, "--quiet"],
                    repository,
                    repository,
                    repository
                )
                .success());

                for dependency in nol.dependencies.iter() {
                    assert!(shell!(
                        dir.as_str(),
                        "cargo",
                        vec!["add", dependency, "--quiet"],
                        dependency,
                        dependency,
                        dependency
                    )
                    .success());
                }
            }
        } else if !Path::new(dir.as_str()).is_dir() {
            assert!(shell!(
                format!("{}/Nol", env!("HOME")).as_str(),
                "cargo",
                vec!["new", "--bin", repository],
                repository,
                repository,
                dir
            )
            .success());

            for dependency in nol.dependencies.iter() {
                assert!(shell!(
                    dir.as_str(),
                    "cargo",
                    vec!["add", dependency],
                    dependency,
                    dependency,
                    dependency
                )
                .success());
            }
        }
        if nol.quiet {
            assert!(shell!(dir.as_str(), "cargo", vec!["run", "--quiet"], dir, dir, dir).success());
        } else {
            assert!(shell!(
                dir.as_str(),
                "cargo",
                vec!["run"],
                dir,
                "Errors has been founded",
                dir
            )
            .success());
        }
    }
    println!();
    exit(0);
}
