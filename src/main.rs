use serde::{Deserialize, Serialize};
use sha1::Sha1;
use std::collections::HashMap;
use std::fs::read;
use std::path::Path;
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(Deserialize, Debug)]
struct Config {
    netlify_auth_token: String,
    netlify_site_id: String,
}

#[derive(StructOpt)]
struct CliFlags {
    #[structopt(parse(from_os_str), long = "path")]
    path: std::path::PathBuf,
}

#[derive(Serialize, Debug)]
struct CreateDeployArgs {
    files: HashMap<String, String>,
    draft: bool,
}

#[derive(Deserialize, Debug)]
struct CreateDeployResponse {
    id: String,
    required: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    };

    let args = CliFlags::from_args();

    let mut hashes = HashMap::new();

    println!("File hashes");
    for entry in WalkDir::new(&args.path) {
        let dir_entry = entry.unwrap();

        if !dir_entry.file_type().is_dir() {
            let path = dir_entry.path();
            let path_suffix = &path.strip_prefix(&args.path.as_path()).unwrap();

            let mut file = read(&path).unwrap();

            let mut hasher = Sha1::new();

            hasher.update(&mut file);

            let hash = hasher.digest().to_string();

            println!("{}: {}", &path_suffix.display(), hash);

            let str = path.to_owned().into_os_string().into_string().unwrap();
            hashes.insert(str, hash);
        }
    }
    println!();

    let create_deploy_args = CreateDeployArgs {
        files: hashes.clone(),
        draft: false,
    };

    dbg!(&create_deploy_args);

    let client: reqwest::Client = reqwest::Client::new();

    let mut auth_headers = reqwest::header::HeaderMap::new();

    auth_headers.insert(
        "Authorization",
        format!("Bearer {}", config.netlify_auth_token)
            .parse()
            .unwrap(),
    );

    let resp_json = client
        .post(format!(
            "https://api.netlify.com/api/v1/sites/{}/deploys",
            config.netlify_site_id
        ))
        .headers(auth_headers)
        .json(&create_deploy_args)
        .send()
        .await?
        .json::<CreateDeployResponse>()
        .await?;

    dbg!(&resp_json);

    let mut reverse_hashes = HashMap::new();

    for (path, hash) in &hashes {
        reverse_hashes.insert(hash, path);
    }

    println!("Files needed to be uploaded: {}", resp_json.required.len());

    for required_hash in resp_json.required {
        let file = reverse_hashes.get(&required_hash).unwrap();

        let required_file_path = &args.path.as_path().join(Path::new(file));

        let file_contents = read(&required_file_path).unwrap();

        let client = reqwest::Client::new();

        let mut put_headers = reqwest::header::HeaderMap::new();

        put_headers.insert(
            "Authorization",
            format!("Bearer {}", config.netlify_auth_token)
                .parse()
                .unwrap(),
        );
        put_headers.insert("Content-Type", "application/octet-stream".parse().unwrap());

        client
            .put(format!(
                "https://api.netlify.com/api/v1/deploys/{}/files/{}",
                resp_json.id, file
            ))
            .headers(put_headers)
            .body(file_contents)
            .send()
            .await?;
    }

    Ok(())
}
