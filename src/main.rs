extern crate image;
extern crate piston_window;

use piston_window::*;

mod raycaster;

fn main() {
    let (width, height) = (320, 240);

    let mut window: PistonWindow = WindowSettings::new(env!("CARGO_PKG_NAME"), [width, height])
        .exit_on_esc(true)
        .graphics_api(OpenGL::V4_2)
        .build()
        .unwrap();

    // Create a simple canvas
    let mut canvas = image::ImageBuffer::new(width, height);
    canvas.put_pixel(100, 100, image::Rgba([0xff, 0, 0, 0xff]));

    // Transform into a texture so piston can use it.
    let mut texture: G2dTexture = Texture::from_image(
        &mut window.create_texture_context(),
        &canvas,
        &TextureSettings::new(),
    )
    .unwrap();

    // Patrick
    let mut environment = raycaster::Environment::default();
    environment.draw(&mut texture);

    // The window event loop.
    window.set_lazy(true);
    while let Some(event) = window.next() {
        let size = window.size();
        window.draw_2d(&event, |context, graphics, _| {
            image(
                &texture,
                context
                    .transform
                    .scale(size.width / width as f64, size.height / height as f64),
                graphics,
            );
        });
    }
}
