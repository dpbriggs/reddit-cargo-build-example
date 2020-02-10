//raytraces an image

//Include
// extern crate minifb; -- not required in rust 2018
use minifb::{Key, Window, WindowOptions};

//Const
const WIDTH: usize = 640;
const HEIGHT: usize = 360;

//STRUCTS

//Triangle
struct Triangle {
    //Points
    v1: Vec<f32>,
    v2: Vec<f32>,
    v3: Vec<f32>,
}
//Mesh
struct Mesh {
    tris: Vec<Triangle>, //Array full of Triangle objects
}
//Ray
struct Ray {
    origin: Vec<f32>,
    direction: Vec<f32>,
}
//FUNCTIONS

//LOOPS

//Main loop
fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
// fn main() {
//     println!("Hello, world!");
// }
