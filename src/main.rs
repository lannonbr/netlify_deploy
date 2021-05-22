use bimap::BiMap;
use color_eyre::eyre::Result;
use futures::future::try_join_all;
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use std::path::{Path, PathBuf};
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
    #[structopt(long)]
    prod: bool,
}

#[derive(Serialize, Debug)]
struct CreateDeployArgs {
    files: BiMap<String, String>,
    draft: bool,
}

#[derive(Deserialize, Debug)]
struct CreateDeployResponse {
    id: String,
    required: Vec<String>,
}

async fn make_hash(path: PathBuf) -> Result<String, Box<dyn std::error::Error + 'static>> {
    let file = tokio::fs::read(path).await?;

    let mut hasher = Sha1::new();

    hasher.update(&file);

    let hash = hasher.digest().to_string();

    Ok(hash)
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = CliFlags::from_args();

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    };
    let mut hashes = BiMap::new();

    let mut hash_futures = vec![];
    let mut strs = vec![];

    for entry in WalkDir::new(&args.path) {
        let dir_entry = entry.unwrap();
        let is_dir = dir_entry.file_type().is_dir();
        let path = dir_entry.path();

        if !is_dir {
            let path_suffix = path.strip_prefix(&args.path.as_path()).unwrap();

            let str = path_suffix
                .to_owned()
                .into_os_string()
                .into_string()
                .unwrap();

            let hash_fut = make_hash(path.to_owned());

            strs.push(str.clone());
            hash_futures.push(hash_fut);
        }
    }

    let resolved_hashes = try_join_all(hash_futures).await.unwrap();

    for (pos, hash) in resolved_hashes.iter().enumerate() {
        hashes.insert(strs.get(pos).unwrap().to_string(), hash.clone());
    }

    let create_deploy_args = CreateDeployArgs {
        files: hashes.clone(),
        draft: !args.prod,
    };

    println!(
        "Uploading {} files to be checked against the deployed site.",
        create_deploy_args.files.len()
    );

    let client: reqwest::Client = reqwest::Client::new();

    let resp_json = client
        .post(format!(
            "https://api.netlify.com/api/v1/sites/{}/deploys",
            config.netlify_site_id
        ))
        .bearer_auth(&config.netlify_auth_token)
        .json(&create_deploy_args)
        .send()
        .await?
        .json::<CreateDeployResponse>()
        .await?;

    println!("Files needed to be uploaded: {}", resp_json.required.len());

    let iter = resp_json.required.clone().into_iter();

    let response = futures::stream::iter(iter)
        .map(|required_hash| {
            let client = &client;
            let auth_token = &config.netlify_auth_token;
            let file = hashes.get_by_right(&required_hash).unwrap().clone();
            let deploy_id = resp_json.id.clone();

            let required_file_path = args.path.as_path().join(Path::new(&file));

            async move {
                let file_contents = tokio::fs::read(required_file_path).await.unwrap();
                client
                    .put(format!(
                        "https://api.netlify.com/api/v1/deploys/{}/files/{}",
                        deploy_id, file
                    ))
                    .header("Content-Type", "application/octet-stream")
                    .bearer_auth(auth_token)
                    .body(file_contents)
                    .send()
                    .await
            }
        })
        .buffer_unordered(5);

    response
        .for_each(|r| async {
            match r {
                Ok(_) => (),
                Err(e) => eprintln!("Error: {}", e),
            }
        })
        .await;

    println!("Deploy successful!");

    Ok(())
}
