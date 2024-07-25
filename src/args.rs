use clap::Parser;

#[derive(Parser)]
#[command(
  name = "Game of Life",
  author = "Michał Czyż",
  version = "1.0.0",
  about = "Rust implementation of Conway's Game of Life",
  long_about = None
)]
pub struct Args {
  #[arg(long, default_value_t = 25)]
  pub width: usize,

  #[arg(long, default_value_t = 25)]
  pub height: usize,

  #[arg(short, long, default_value_t = 0.5)]
  pub probability: f64,

  #[arg(short, long, default_value_t = 0)]
  pub seed: u64,

  #[arg(short, long, default_value_t = 15)]
  pub tickrate: u64,

  #[arg(long, default_value_t = String::from(""))]
  pub load: String,

  #[arg(long, default_value_t = String::from(""))]
  pub save: String,

  #[arg(short, long, default_value_t = 3)]
  pub repopulation: usize,

  #[arg(short, long, default_value_t = 2)]
  pub overpopulation: usize,

  #[arg(short, long, default_value_t = 3)]
  pub underpopulation: usize,
}