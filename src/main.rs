use crate::core::run::run;

mod core;
mod constants;

#[macroquad::main("Ven")]
async fn main() {
  run().await
}