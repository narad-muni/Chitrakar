use std::{cmp::max, fs::File, io::{BufWriter, Write}};

#[derive(Clone, Debug)]
struct Pixel {
    r: f32,
    g: f32,
    b: f32,
}

fn render() {
    // Image size
    let width: f32 = 1024.0;
    let height: f32 = 768.0;

    // Create ouput img file
    let img = File::create("./rs.ppm").unwrap();
    let mut buf = BufWriter::new(img);
    // Write to image
    buf.write(format!("P3\n{} {}\n255\n", width, height).as_bytes()).unwrap();

    // Framebuffer with h*w no. of Pixel objects
    let mut frame_buffer: Vec<Pixel> = Vec::with_capacity((height * width) as usize);

    // Initialize empty framebuffer with gradient
    for i in 1..height as i32+1 {
        for j in 1..width as i32+1 {
            let px = Pixel{
                r: ((j ^ i) as f32) / (height as f32),
                g: ((i & j) as f32) * (width as f32),
                b: (i * j) as f32,
            };

            buf.write(format!("{} {} {}\n",
                (255 as f32 * px.r) as u8,
                (255 as f32 * px.g) as u8,
                (255 as f32 * px.b) as u8,
            ).as_bytes()).unwrap();
            
            frame_buffer.push(px);
        }
    }

}

fn main() {
    render();
    // Find image rs.ppm
    // Open it on https://www.cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html
}
