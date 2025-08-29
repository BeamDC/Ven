use crate::core::run::run;

mod core;
mod constants;
mod bindings;

#[macroquad::main("Ven")]
async fn main() {
  run().await
}