use minifb::{Key, Window, WindowOptions};

mod framebuffer;
mod game_of_life;

use crate::framebuffer::Framebuffer;
use crate::game_of_life::GameOfLife;

fn main() {
    // Tamaño de la ventana
    let window_width = 800;
    let window_height = 600;

    // Resolución lógica del Game of Life
    let framebuffer_width = 100;
    let framebuffer_height = 75;

    let mut framebuffer =
        Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    // El framebuffer se limpia solamente una vez.
    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    // Crear el juego
    let mut game = GameOfLife::new(
        framebuffer_width,
        framebuffer_height,
    );

    // Crear nuestra escena inicial
    game.create_final_scene();

    // Velocidad de la simulación
    window.set_target_fps(10);

    // Game loop
    while window.is_open() && !window.is_key_down(Key::Escape) {

        // -------------------------
        // DIBUJAR CÉLULAS MUERTAS
        // -------------------------

        framebuffer.set_current_color(0x000000);

        for y in 0..framebuffer_height {
            for x in 0..framebuffer_width {
                if !game.cells[y][x] {
                    framebuffer.point(x, y);
                }
            }
        }

        // -------------------------
        // DIBUJAR CÉLULAS VIVAS
        // -------------------------

        framebuffer.set_current_color(0xFFFFFF);

        for y in 0..framebuffer_height {
            for x in 0..framebuffer_width {
                if game.cells[y][x] {
                    framebuffer.point(x, y);
                }
            }
        }

        // -------------------------
        // MOSTRAR FRAMEBUFFER
        // -------------------------

        window
            .update_with_buffer(
                &framebuffer.buffer,
                framebuffer_width,
                framebuffer_height,
            )
            .unwrap();

        // -------------------------
        // SIGUIENTE GENERACIÓN
        // -------------------------

        game.next_generation();
    }
}