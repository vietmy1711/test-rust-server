use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use git2::{Cred, RemoteCallbacks, Repository};
use std::env;
use std::fs::remove_dir_all;
use std::path::Path;

const MIRRORED_GIT_URL: &str = "git@github.com:vietmy1711/test-app-mirrored.git";

const MIRRORED_LOCAL_URL: &str = "/Users/GJ376GXGQ0/development/test-app-mirrored";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let path = Path::new(MIRRORED_LOCAL_URL);

    if path.exists() {
        remove_dir_all(MIRRORED_LOCAL_URL).expect("fail to delete dir");
    }

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_ed25519", env::var("HOME").unwrap())),
            None,
        )
    });

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    // Clone the project.
    let _repo: Repository = match builder.clone(MIRRORED_GIT_URL, path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    HttpServer::new(|| App::new().service(branch_commit))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[post("/branch-commit")]
async fn branch_commit(branch_name: String) -> impl Responder {
    HttpResponse::Ok().body(branch_name)
}
