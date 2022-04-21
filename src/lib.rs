use std::collections::BTreeMap;
use std::process::exit;


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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
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

pub struct PixelList {
    list: Vec<Pixel>,
    dimensions: Dimensions,
}

impl PixelList {
    pub fn new(array: &[u8], dimensions: Dimensions) -> Self {
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

    fn get_passable_neighbours(&self, point: &Point ) -> MazeNode {
        let mut node_update = MazeNode::new();
    
        if point.y > 0 {
            let north_point = Point {
                x: point.x,
                y: point.y - 1,
            };
    
            if self.is_passable(&north_point) {
                node_update.conections.north = Some(north_point);
            }
        }
        if point.y + 1 < self.dimensions.height.try_into().unwrap() {
            let south_point = Point {
                x: point.x,
                y: point.y + 1,
            };
    
            if self.is_passable(&south_point) {
                node_update.conections.south = Some(south_point);
            }
        }
        if point.x > 0 {
            let west_point = Point {
                x: point.x - 1,
                y: point.y,
            };
    
            if self.is_passable(&west_point) {
                node_update.conections.west = Some(west_point);
            }
        }
        if point.x + 1 < self.dimensions.width.try_into().unwrap() {
            let east_point = Point {
                x: point.x + 1,
                y: point.y,
            };
    
            if self.is_passable(&east_point) {
                node_update.conections.east = Some(east_point);
            }
        }
    
        node_update
    }
}

#[derive(Clone, Copy, Debug)]
struct MazeNode {
    conections: Connectors,
    passable: u8,
}

impl Default for MazeNode {
    fn default() -> Self {
        Self::new()
    }
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

pub struct Maze {
    dimensions: Dimensions,
    nodes: BTreeMap<Point, MazeNode>,
}

impl Maze {
    pub fn new(dimensions: Dimensions, pixel_list: &PixelList) -> Self {
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
                    passable: Maze::get_from_bool(pixel.passable()),
                };

                if pixel.passable() {
                    node = pixel_list.get_passable_neighbours(&point);
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

    pub fn print_maze(&self) {
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

    // look around the box edges, return passable
    pub fn find_start(&self) -> Vec<Point> {
        let mut entrace_list: Vec<Point> = Vec::new();
        let width: usize = self.dimensions.width.try_into().unwrap();
        let height: usize = self.dimensions.height.try_into().unwrap();

        // top and bottom
        for y in [0,height-1] {
            for x in 0..width {
                let current_location = Point { x, y };
                let node = match self.nodes.get(&current_location) {
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
            for y in 0..height {
                let current_location = Point { x, y };
                let node = match self.nodes.get(&current_location) {
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

    pub fn solve_maze(&self, start: &Point, end: &Point, last: &Point) {
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

    fn get_from_bool(x: bool) -> u8 {
        match x {
            true => 1,
            false => 0,
        }
    }
}