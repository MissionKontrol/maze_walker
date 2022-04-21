use std::collections::BTreeMap;
use std::fs::File;
use std::process::exit;

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

    let entrances = find_start(&maze);
    println!("Entrances {:?} {:?}", entrances[0], entrances[1]);
    maze.solve_maze( &entrances[0], &entrances[1], &entrances[1]);
}



#[derive(Clone, Copy, Debug)]
struct Connectors {
    north: Option<Point>,
    east: Option<Point>,
    south: Option<Point>,
    west: Option<Point>,
}

impl Connectors {
    fn new() -> Self {
        Connectors {
            north: None,
            south: None,
            east: None,
            west: None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct MazeNode {
    conections: Connectors,
    passable: u8,
}

impl MazeNode {
    fn new() -> Self {
        MazeNode {
            conections: Connectors::new(),
            passable: 0,
        }
    }

    fn is_passable(&self) -> bool {
        if self.passable == 0 {
            return false;
        }
        true
    }

    fn get_connections(&self) -> Option<Vec<Point>> {
        let mut connections: Vec<Point> = Vec::with_capacity(4); // cardinal directions
        if let Some(node) = self.conections.north {
            connections.push(node);
        }
        if let Some(node) = self.conections.east {
            connections.push(node);
        }
        if let Some(node) = self.conections.south {
            connections.push(node);
        }
        if let Some(node) = self.conections.west {
            connections.push(node);
        }
        
        if !connections.is_empty() {
            Some(connections)
        }
        else { None }
    }
}

impl Default for MazeNode {
    fn default() -> Self {
        Self::new()
    }
}

struct Maze {
    dimensions: Dimensions,
    nodes: BTreeMap<Point, MazeNode>,
}

impl Maze {
    fn new(dimensions: Dimensions, pixel_list: &PixelList) -> Self {
        let mut maze = Maze {
            dimensions,
            nodes: BTreeMap::new(),
        };

        let node_insert_list: Vec<(Point, MazeNode)> = pixel_list
            .list
            .iter()
            .map(|pixel| -> (Point, MazeNode) {
                let point = pixel.point;

                let mut node = MazeNode {
                    conections: Connectors::new(),
                    passable: get_from_bool(pixel.passable()),
                };

                if pixel.passable() {
                    node = get_passable_neighbours(&point, pixel_list);
                    node.passable = 1;
                }

                (point, node)
            })
            .collect();

        let _foo = node_insert_list
            .iter()
            .flat_map(|(point, node)| maze.nodes.insert(*point, *node))
            .collect::<Vec<MazeNode>>();

        maze
    }

    fn print_maze(&self) {
        for y in 0..self.dimensions.height {
            for x in 0..self.dimensions.width {
                let node = self
                    .nodes
                    .get(&Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    })
                    .unwrap();
                if node.is_passable() {
                    print!(" ");
                } else {
                    print!("*");
                }
            }
            println!();
        }
    }

    fn solve_maze(&self, start: &Point, end: &Point, last: &Point) {
        if start == end {
            println!("Found exit: {end:?}");
            exit(0)
        }
    
        let current_node = self.nodes.get(start).unwrap();
        if let Some(connection_points) = current_node.get_connections() {
            for point in connection_points.iter() {
                println!("point: {point:?}  start: {start:?}");
                if point == last { continue; }
    
                let last = start;
                self.solve_maze( point, end, last)
            }
        }
    }
    
}

fn get_passable_neighbours(point: &Point, pixel_list: &PixelList) -> MazeNode {
    let mut node_update = MazeNode::new();

    if point.y > 0 {
        let north_point = Point {
            x: point.x,
            y: point.y - 1,
        };

        if pixel_list.is_passable(&north_point) {
            node_update.conections.north = Some(north_point);
        }
    }
    if point.y + 1 < pixel_list.dimensions.height.try_into().unwrap() {
        let south_point = Point {
            x: point.x,
            y: point.y + 1,
        };

        if pixel_list.is_passable(&south_point) {
            node_update.conections.south = Some(south_point);
        }
    }
    if point.x > 0 {
        let west_point = Point {
            x: point.x - 1,
            y: point.y,
        };

        if pixel_list.is_passable(&west_point) {
            node_update.conections.west = Some(west_point);
        }
    }
    if point.x + 1 < pixel_list.dimensions.width.try_into().unwrap() {
        let east_point = Point {
            x: point.x + 1,
            y: point.y,
        };

        if pixel_list.is_passable(&east_point) {
            node_update.conections.east = Some(east_point);
        }
    }

    node_update
}

fn summarize(info: &OutputInfo) {
    println!("width {} * height {}", info.width, info.height);
    println!(
        "bit depth {:?}, line size {}",
        info.bit_depth, info.line_size
    );
    println!("colour type {:?}", info.color_type);
}

// look around the box edges, return passable
fn find_start(maze: &Maze) -> Vec<Point> {
    let mut entrace_list: Vec<Point> = Vec::new();
    let width: usize = maze.dimensions.width.try_into().unwrap();
    let height: usize = maze.dimensions.height.try_into().unwrap();

    // top and bottom
    for y in [0,height-1] {
        for x in 0..width {
            let current_location = Point { x, y };
            let node = match maze.nodes.get(&current_location) {
                Some(pixel) => pixel,
                None => panic!(
                    "invalid pixel index {},{}",
                    current_location.x, current_location.y
                ),
            };

            if node.is_passable() {
                entrace_list.push(current_location);
            }
        }
    }

    for x in [0,width-1] {
        for y in 0..width {
            let current_location = Point { x, y };
            let node = match maze.nodes.get(&current_location) {
                Some(pixel) => pixel,
                None => panic!(
                    "invalid pixel index {},{}",
                    current_location.x, current_location.y
                ),
            };

            if node.is_passable() {
                entrace_list.push(current_location);
            }
        }
    }

    entrace_list
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

#[derive(Default, Clone)]
struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
    point: Point,
}

impl Pixel {
    fn passable(&self) -> bool {
        if self.red > 0 || self.green > 0 || self.blue > 0 {
            return true;
        }
        false
    }
}

struct PixelList {
    list: Vec<Pixel>,
    dimensions: Dimensions,
}

impl PixelList {
    fn new(array: &[u8], dimensions: Dimensions) -> Self {
        const RGBA: usize = 4;
        let mut pixel_list = Vec::with_capacity(array.len() / RGBA);

        array
            .iter()
            .enumerate()
            .fold(Pixel::default(), |mut acc: Pixel, (i, byte)| {
                match (i + 1) % RGBA {
                    0 => {
                        acc.alpha = *byte;
                        let line_size: usize = dimensions.width.try_into().unwrap();
                        acc.point = Point {
                            x: (i / RGBA) % line_size,
                            y: i / (line_size * RGBA),
                        };
                        pixel_list.push(acc.clone())
                    }
                    1 => acc.red = *byte,
                    2 => acc.green = *byte,
                    3 => acc.blue = *byte,
                    _ => panic!("inconcievable RGBA value {i}"),
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
        self.get_point(point).passable()
    }

    fn get_point(&self, point: &Point) -> &Pixel {
        let index: usize = self.to_index(point);
        self.list.get(index).unwrap()
    }

    fn to_index(&self, point: &Point) -> usize {
        let width: usize = self.dimensions.width.try_into().unwrap();
        point.y * width + point.x
    }
}


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
struct Point {
    x: usize,
    y: usize,
}
