use std::fs::File;

fn main() {
    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
    let decoder = png::Decoder::new(File::open("mazes/maze.png").unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();
    
    println!("width {} * height {}", info.width, info.height);
    println!("bit depth {:?}, line size {}", info.bit_depth, info.line_size);

    println!("colour type {:?}", info.color_type);

    let pixel_list = vec!(Pixel::default();info.buffer_size());
    
    // Grab the bytes of the image.
    let bytes = &buf[..info.buffer_size()];

    // Inspect more details of the last read frame.
    const RGBA: u32 = 4;
    bytes.iter().fold(0, |acc:u32, b| {
        if (acc + 1) % 1 == 0 {
            if (acc % info.width) == 0  && acc > 0 {
                println!("{}", b);
            }
            else {
                print!("{}", b);
            }
        } 
        acc + 1
    });
    // let in_animation = reader.info().frame_control.is_some();
}

#[derive(Clone)]
struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Default for Pixel {
    fn default() -> Pixel {
        Pixel { red: 0, green: 0, blue: 0, alpha: 0 }
    }
}