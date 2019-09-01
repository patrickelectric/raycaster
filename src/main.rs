extern crate piston_window;
extern crate image;

mod raycaster;

use piston_window::*;

fn main() {
    let (width, height) = (320, 240);
    let map = [
        [1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1],
    ];

    let mut window: PistonWindow =
        WindowSettings::new(env!("CARGO_PKG_NAME"), [width, height])
        .exit_on_esc(true).graphics_api(OpenGL::V4_2).build().unwrap();

    // Create a simple canvas
    let mut canvas = image::ImageBuffer::new(width, height);
    canvas.put_pixel(100, 100, image::Rgba([0xff, 0, 0, 0xff]));

    // Transform into a texture so piston can use it.
    let texture: G2dTexture = Texture::from_image(
        &mut window.create_texture_context(),
        &canvas,
        &TextureSettings::new()
    ).unwrap();

    // The window event loop.
    window.set_lazy(true);
    while let Some(event) = window.next() {
        let size = window.size();
        window.draw_2d(&event, |context, graphics, _| {
            image(&texture,
            context.transform.scale(size.width/width as f64, size.height/height as f64),
            graphics);
        });
        println!("p");
    }
}
