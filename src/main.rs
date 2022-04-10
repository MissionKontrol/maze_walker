use std::{fs::File};
use std::collections::BTreeMap;

use png::{OutputInfo};

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
    let dimensions = Dimensions { width: info.width, height: info.height};
    let pixel_list = PixelList::new(bytes, dimensions);
    println!("Pixel list length {}", pixel_list.list.len());
    // let pixel_list = pixel_list_from_array(bytes);
    
    let maze = Maze::new(
        Dimensions { width: info.width, height: info.height},
        &pixel_list
        );

    // let entrances = find_start(&pixel_list);
    // for entry in entrances.iter() {
    //     println!("Entry @ {},{}", entry.x, entry.y);
    // }
}

fn solve_maze(maze: Vec<Pixel>, start: Point) {
    let start_nodes = find_start(&maze);
    
    // arbitraily use first node
    let start = &start_nodes[0];
    let position_index = |point: &Point| point.y * 41 + point.x;

}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
struct MazeNode {
    conections: Connectors,
    passable: u8,
}

impl MazeNode {
    fn new() -> Self {
        MazeNode { conections: Connectors::new(), passable: 0 }
    }
    
    fn is_passable(&self) -> bool {
        if 0 == self.passable {
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
    dimensions: Dimensions,
    nodes: BTreeMap<Point,MazeNode>,
}

impl Maze {
    fn new(dimensions: Dimensions, pixel_list: &PixelList) -> Self {
        let mut maze = Maze { dimensions, nodes: BTreeMap::new() };

        let node_insert_list: Vec<(Point,MazeNode)> = pixel_list.list.iter().enumerate().map(|(index, pixel)| -> (Point,MazeNode) {
            let point = pixel.point;
            let node = MazeNode { 
                conections: Connectors::new(),
                passable: get_from_bool(pixel.passable()),
            };

            (point,node)}).collect();

        let mut node_tree: BTreeMap<Point,MazeNode> = BTreeMap::new();
        let _insert_result = node_insert_list.iter().map( |node| {
            node_tree.insert(node.0, node.1)
        }).collect::<Vec<Option<MazeNode>>>();

        for (point, node) in &node_tree {
            if node.is_passable(){
                let mut neighbours: Vec<Direction> = Vec::new();
                if point.y > 0 {
                    neighbours.push(Direction::North);
                }
                if point.y < dimensions.height.try_into().unwrap() {
                    neighbours.push(Direction::South);
                }
                if point.x > 0 { neighbours.push(Direction::West);}
                if point.x < dimensions.width.try_into().unwrap() { 
                    neighbours.push(Direction::East);
                }

                let x: Vec<(&Point, &Direction)> = neighbours.iter().map(|direction| -> Option<(&Point, &Direction)> {
                    let neighbour_point = match direction {
                        Direction::North => Point{ x: point.x, y: point.y - 1 },
                        Direction::South => Point{ x: point.x, y: point.y + 1 },
                        Direction::East => Point{ x: point.x + 1, y: point.y },
                        Direction::West => Point{ x: point.x - 1, y: point.y },
                    };
                    if pixel_list.is_passable( &neighbour_point ) {
                        Some(( point,direction ))
                    }
                    else { None }
                }).flatten().collect();

                x.iter().map(|x| {
                    println!("{:?} {:?}", x.0, x.1);

                });
            }
        }

        maze.nodes = node_tree;
        maze
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn get_from_bool(x: bool) -> u8 {
    match x {
        true => 1,
        false => 0,
    }
}

#[derive(Clone, Copy)]
struct Dimensions {
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

impl Pixel {
    fn passable(&self) -> bool {
        if self.red > 0 ||
        self.green > 0 ||
            self. blue > 0 { return false }
        true
    }
}

impl Default for Pixel {
    fn default() -> Pixel {
        Pixel { red: 0, green: 0, blue: 0, alpha: 0, point: Default::default() }
    }
}

struct PixelList {
    list: Vec<Pixel>,
    dimensions: Dimensions,
}

impl PixelList {
    fn new(array: &[u8], dimensions: Dimensions) -> Self {
        const RGBA: usize = 4;
        let mut pixel_list = Vec::with_capacity(array.len()/RGBA);
    
        array.iter().enumerate().fold(Pixel::default(), |mut acc: Pixel, (i,byte)| {
            match (i + 1) % RGBA {
                0 => {  acc.alpha = *byte;
                        let line_size: usize = dimensions.width.try_into().unwrap();
                        acc.point = Point{ x: ((i / RGBA) % line_size ), y: i % (line_size/RGBA)};  
                        pixel_list.push(acc.clone())},
                1 => acc.red = *byte,
                2 => acc.green = *byte,
                3 => acc.blue = *byte,
                _ => panic!("inconcievable RGBA value {i}")
            };
    
            acc
        });
        
        PixelList { 
            dimensions: Dimensions {
                width: dimensions.width,
                height: dimensions.height,
            },
            list: pixel_list,
        }
    }

    fn is_passable(&self, point: &Point) -> bool {
        print!("{point:?} ");
        self.get_point(point).passable()
    }

    fn get_point(&self, point: &Point) -> &Pixel {
        let index: usize = self.to_index(point);
        println!("{index}");
        self.list.get(index).unwrap()
    }

    fn to_index(&self, point: &Point ) -> usize {
        let width: usize = self.dimensions.width.try_into().unwrap();
        point.y * width + point.x
    }
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