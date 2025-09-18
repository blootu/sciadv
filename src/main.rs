use clap::Parser;
use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static VALID_TITLES: Lazy<HashMap<&'static str, Option<&'static str>>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert("steins_gate", Some("Steins;Gate"));

    m.insert("chaos_head", None);
    m.insert("robotics_notes", None);
    m.insert("chaos_child", None);
    m.insert("occultic_nine", None);
    m.insert("anonymous_code", None);

    m
});

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    title: String,
}

fn main() {
    let args = Args::parse();

    let query = args.title.as_str();

    match VALID_TITLES.get(query) {
        Some(Some(stylized)) => {
            println!("\"{stylized}\" is supported");
        }
        Some(None) => {
            println!("\"{query}\" is known but not yet implemented");
        }
        None => {
            println!("\"{query}\" is not a recognised title");
        }
    }
}
