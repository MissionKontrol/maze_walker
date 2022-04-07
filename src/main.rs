use std::{fs::File, default};
use std::collections::BTreeMap;

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
    
    let maze = Maze::new(
        MazeDimensions { width: info.width,height: info.height},
        &pixel_list
        );

    let entrances = find_start(&pixel_list);
    for entry in entrances.iter() {
        println!("Entry @ {},{}", entry.x, entry.y);
    }
    // let in_animation = reader.info().frame_control.is_some();
}

fn to_index(point: &Point, width: usize ) -> usize {
    point.y * width + point.x
}

fn solve_maze(maze: Vec<Pixel>, start: Point) {
    let start_nodes = find_start(&maze);
    
    // arbitraily use first node
    let start = &start_nodes[0];
    let position_index = |point: &Point| point.y * 41 + point.x;

}

struct Connectors {
    north: bool, 
    east: bool,
    south: bool,
    west: bool,
}

impl Connectors {
    fn new() -> Self {
        Connectors {
            north: false,
            south: false,
            east: false,
            west: false,
        }
    }
}

struct MazeNode {
    conections: Option<Point>,
    passable: u8,
}

impl MazeNode {
    fn new() -> Self {
        MazeNode { conections: None, passable: 0 }
    }
    
    fn is_passable(&self) -> bool {
        if let 0 = self.passable {
            false
        } else {
            true
        }
    }
}

impl Default for MazeNode {
    fn default() -> Self {
        Self::new()
    }
}

struct Maze {
    dimensions: MazeDimensions,
    nodes: BTreeMap<Point,MazeNode>,
}

impl Maze {
    fn new(dimensions: MazeDimensions, pixel_list: &Vec<Pixel>) -> Self {
        let mut maze = Maze { dimensions, nodes: BTreeMap::new() };

        let _node_insert_list: Vec<Option<MazeNode>> = pixel_list.iter().map(|pixel| -> Option<MazeNode> 
            { maze.nodes.insert(pixel.point,
                MazeNode { 
                    passable: get_from_bool(pixel.passable()),
                    conections: None,
            })}).collect();

        let maze = maze.try_connect(pixel_list);
        maze
    }

    fn try_connect(mut self, pixel_list: &Vec<Pixel>) -> Self {
        for y in 0..=self.dimensions.height as usize {
            for x in 0..=self.dimensions.width as usize {
                let point = Point{x,y};
                let node = self.nodes.get(&point).unwrap();
                let neighbour_points = self.get_connections(&point, pixel_list);


            }
        }
        self
    }

    fn get_connections(&self, origin: &Point, pixel_list: &Vec<Pixel>) -> Connectors {
        let mut connectors = Connectors::new();

        let north = {
            if origin.y == 0 { connectors.north = false }
            else { 
                
                connectors.north = false }
        };

        let south = ||{
            if origin.y == self.dimensions.height as usize { connectors.south = false }
            else { connectors.south = true }
        };
        
        let west = ||{
            if origin.x == 0 { connectors.west = false }
            else {  connectors.west = true }
        };
        
        let east = ||{
            if origin.x == self.dimensions.width as usize { connectors.east = false }
            else { connectors.east = true }
        };
        
        connectors
    }

    fn get_north_point(&self, origin: &Point, pixel_list: &Vec<Pixel> ) -> Option<MazeNode> {
        let mazenode = MazeNode::new();

        let north_point = if origin.y > 0 { Some(Point { x: origin.x, y: origin.y + 1 })}
                                        else { None };

        todo!()
    }

    fn get_neighbour(&self, neighbour: Option<Point>) -> Option<&MazeNode> {
        if let Some(point) = neighbour {
            if let Some((_key, node)) = self.nodes.get_key_value(&point) {
                Some(node)
            }
            else { None }
        }
        else { None }
    }
}
fn get_from_bool(x: bool) -> u8 {
            match x {
                true => 1,
                false => 0,
            }
        }

struct MazeDimensions {
    width: u32,
    height: u32,
}


#[derive(Clone)]
struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
    point: Point,
}

struct MazeSolution {
    start: Point,
    solution_tree: Vec<MazeSolutionNode>,
}

struct MazeSolutionNode {
    point: Point,
    child: Option<Box<MazeSolutionNode>>,
    child_cost: u32,
    peer: Option<Box<MazeSolutionNode>>,
    peer_cost: u32,
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
fn find_start(maze: &Vec<Pixel>) -> Vec<Point> {
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Default for Point {
    fn default() -> Point {
        Point { x: 0, y: 0 }
    }
}