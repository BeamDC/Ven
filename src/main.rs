use crate::core::run::run;

mod core;


#[macroquad::main("MyGame")]
async fn main() {
  run().await
}