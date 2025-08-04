use macroquad::prelude::*;
use macroquad::rand::rand;
use crate::core::ui::button::Button;
use crate::core::ui::interaction::MouseInteract;
use crate::core::ui::object::Object;
use crate::core::ui::toolbar::Toolbar;

pub async fn run() {
    let mut toolbar = Toolbar::new(500.0, 50.0);
    loop {
        clear_background(BLACK);

        toolbar.handle_input();
        toolbar.draw();

        next_frame().await
    }
}