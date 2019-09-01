extern crate image;
extern crate piston_window;

use piston_window::*;

mod raycaster;

fn main() {
    let (width, height) = (320, 240);

    let opengl_version = OpenGL::V4_2;
    let mut window: PistonWindow = WindowSettings::new(env!("CARGO_PKG_NAME"), [width, height])
        .exit_on_esc(true)
        .graphics_api(opengl_version)
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
    //environment.draw(&mut texture);

    // The window event loop.
    window.set_lazy(true);
    while let Some(event) = window.next() {
        MouseCursorEvent::mouse_cursor(&event, |mouse_pos| {
            //println!("{:#?}", mouse_pos);
        });

        window.draw_2d(&event, |mut context, mut graphics, _| {
            let size = context.viewport.unwrap().window_size;
            environment.draw(&mut context, &mut graphics);
            image(
                &texture,
                context
                    .transform
                    .scale(size[0] / width as f64, size[1] / height as f64),
                graphics,
            );
        });
    }
}
