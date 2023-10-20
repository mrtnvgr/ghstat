use crate::api::get_user_data;
use clap::Parser;
use std::{env, fs, path::PathBuf};
use upon::Engine;

mod api;

#[derive(Parser)]
struct Args {
    template: PathBuf,
}

fn main() {
    let gh_token = env::var("GITHUB_TOKEN").expect("Missing GITHUB_TOKEN env var");
    let login = env::var("GITHUB_LOGIN").expect("Missing GITHUB_LOGIN env var");

    let args = Args::parse();

    let user_data = get_user_data(&gh_token, login);

    let text = fs::read_to_string(args.template).expect("Failed to read template");

    let engine = Engine::new();

    let template = engine.compile(&text).expect("Failed to compile template");
    let rendered_text = template.render(user_data).to_string().unwrap();

    print!("{rendered_text}");
}
