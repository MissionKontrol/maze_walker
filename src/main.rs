use std::{fs::File, default};

use png::{Info, OutputInfo};

fn main() {
    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
    let decoder = png::Decoder::new(File::open("mazes/maze-smallpixel.png").unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();
    summarize(&info);

    // Grab the bytes of the image.
    let bytes = &buf[..info.buffer_size()];
    let pixel_list = pixel_list_from_array(bytes);
    // for (i, j) in pixel_list.iter().enumerate() {
        // if j.red > 0 {
            // print!(" {:?}", j.point);
        // }
        // else {

        //     print!("#");
        // }

        // if (i + 1) % 41 == 0 && i > 0 {
        //     println!();
        // }
    // }

    let entrances = find_start(pixel_list);
    for entry in entrances.iter() {
        println!("{},{}", entry.x, entry.y);
    }
    // let in_animation = reader.info().frame_control.is_some();
}

#[derive(Clone)]
struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
    point: Point,
}

impl Pixel {
    fn passable(&self) -> bool {
        if self.red > 0 ||
            self.green > 0 ||
            self. blue > 0 { return true }
        false
    }
}

impl Default for Pixel {
    fn default() -> Pixel {
        Pixel { red: 0, green: 0, blue: 0, alpha: 0, point: Default::default() }
    }
}

fn pixel_list_from_array(array: &[u8]) -> Vec<Pixel> {
    let mut pixel_list = Vec::with_capacity(array.len()/4);


    // Inspect more details of the last read frame.
    const RGBA: usize = 4;
    array.iter().enumerate().fold(Pixel::default(), |mut acc: Pixel, (i,byte)| {
        match (i + 1) % RGBA {
            0 => {  acc.alpha = *byte;
                    acc.point = Point{ x: ((i / 4) % 41), y: i / 164};  // row = 4 bytes * 41
                    pixel_list.push(acc.clone())},
            1 => acc.red = *byte,
            2 => acc.green = *byte,
            3 => acc.blue = *byte,
            _ => panic!("inconcievable RGBA value {i}")
        };

        acc
    });

    pixel_list
}

fn summarize(info: &OutputInfo) {
    println!("width {} * height {}", info.width, info.height);
    println!("bit depth {:?}, line size {}", info.bit_depth, info.line_size);
    println!("colour type {:?}", info.color_type);
}

// look around the box edges, return passable
fn find_start(maze: Vec<Pixel>) -> Vec<Point> {
    let mut entrace_list: Vec<Point> = Vec::new();
    let index = |point: &Point| point.y * 41 + point.x;

    // top and bottom
    for y in [0,40] {
        for x in 0..41 {
            let current_location = Point { x, y };
            let pixel = match maze.get(index(&current_location)){
                Some(pixel) => pixel,
                None => panic!("invalid pixel index {},{}", current_location.x, current_location.y),
            };

            if pixel.passable() {
                entrace_list.push(current_location);
            }
        }
    }

    for x in [0,40] {
        for y in 0..41 {
            let current_location = Point { x, y };
            let pixel = match maze.get(index(&current_location)){
                Some(pixel) => pixel,
                None => panic!("invalid pixel index {},{}", current_location.x, current_location.y),
            };

            if pixel.passable() {
                entrace_list.push(current_location);
            }
        }
    }

    entrace_list
}

#[derive(Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Default for Point {
    fn default() -> Point {
        Point { x: 0, y: 0 }
    }
}