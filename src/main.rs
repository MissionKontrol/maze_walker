use std::fs::File;
use maze_walker::*;
use png::OutputInfo;

fn main() {
    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
    let decoder = png::Decoder::new(File::open("mazes/maze(9).png").unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();
    summarize(&info);

    // Grab the bytes of the image.
    let bytes = &buf[..info.buffer_size()];
    let dimensions = Dimensions {
        width: info.width,
        height: info.height,
    };
    let pixel_list = PixelList::new(bytes, dimensions);

    let maze = Maze::new(
        Dimensions {
            width: info.width,
            height: info.height,
        },
        &pixel_list,
    );

    maze.print_maze();

    let entrances = maze.find_start();
    println!("Entrances {:?} {:?}", entrances[0], entrances[1]);
    maze.solve_maze( &entrances[0], &entrances[1], &entrances[1]);
}

fn summarize(info: &OutputInfo) {
    println!("width {} * height {}", info.width, info.height);
    println!(
        "bit depth {:?}, line size {}",
        info.bit_depth, info.line_size
    );
    println!("colour type {:?}", info.color_type);
}