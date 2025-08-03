use macroquad::prelude::*;
use macroquad::rand::rand;
use crate::core::ui::button::Button;
use crate::core::ui::interaction::MouseInteract;


pub async fn run() {
    let mut b = Button::new(
        50.0, 50.0,
        160.0, 50.0,
        3.0, String::from("qwertyuiopasdfghjkl")
    );
    b.fill = Color::from_rgba(255,255,255,255);
    b.stroke = Color::from_rgba(255, 0, 0, 255);
    b.text_colour = Color::from_rgba(0, 255, 0, 255);
    b.text_size = 25;

    loop {
        clear_background(BLACK);

        b.draw();
        b.on_press_mut(MouseButton::Left, |button| {
            let r: u8 = (rand() % 256) as u8;
            let g: u8 = (rand() % 256) as u8;
            let b: u8 = (rand() % 256) as u8;
            button.fill = Color::from_rgba(r,g,b,255);

            let r: u8 = (rand() % 256) as u8;
            let g: u8 = (rand() % 256) as u8;
            let b: u8 = (rand() % 256) as u8;
            button.stroke = Color::from_rgba(r,g,b,255);

            let r: u8 = (rand() % 256) as u8;
            let g: u8 = (rand() % 256) as u8;
            let b: u8 = (rand() % 256) as u8;
            button.text_colour = Color::from_rgba(r,g,b,255);
        });
        next_frame().await
    }
}