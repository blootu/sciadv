mod models;
mod data;
mod tui;

use clap::Parser;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use anyhow::Result;

pub static VALID_TITLES: Lazy<HashMap<&'static str, Option<&'static str>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("chaos_head", Some("Chaos;Head"));

    m.insert("steins_gate", None);
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

fn main() -> Result<()> {
    let args = Args::parse();

    let query = args.title.as_str();

    match VALID_TITLES.get(query) {
        Some(Some(stylized)) => {
            if query == "chaos_head" {
                tui::run_app()?;
            } else {
                println!("\"{stylized}\" is known but not yet implemented");
            }
        }
        Some(None) => {
            println!("\"{query}\" is known but not yet implemented");
        }
        None => {
            println!("\"{query}\" is not a recognised title");
            println!("Available titles: chaos_head");
        }
    }

    Ok(())
}
