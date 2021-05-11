use serde::{Deserialize, Serialize};
use sha1::Sha1;
use std::collections::HashMap;
use std::fs::read;
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(Deserialize, Debug)]
struct Config {
    netlify_auth_token: String,
}

#[derive(StructOpt)]
struct CliFlags {
    #[structopt(parse(from_os_str), long = "path")]
    path: std::path::PathBuf,
}

#[derive(Serialize)]
struct CreateDeployArgs {
    files: HashMap<String, String>,
}

fn main() -> std::result::Result<(), std::io::Error> {
    match envy::from_env::<Config>() {
        Ok(config) => println!("{:#?}", config),
        Err(error) => eprintln!("{:#?}", error),
    }

    let args = CliFlags::from_args();

    let mut hashes = HashMap::new();

    for entry in WalkDir::new(&args.path) {
        let dir_entry = entry.unwrap();

        if !&dir_entry.file_type().is_dir() {
            let path = &dir_entry.path();
            let path_suffix = &path.strip_prefix(&args.path.as_path()).unwrap();

            let mut file = read(&path)?;

            let mut hasher = Sha1::new();

            hasher.update(&mut file);

            let hash = hasher.digest().to_string();

            println!("{}: {}", &path_suffix.display(), hash);

            let str = path.to_str().unwrap();
            hashes.insert(str, hash);
        }
    }

    Ok(())
}
