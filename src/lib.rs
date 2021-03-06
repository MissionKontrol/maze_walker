use std::{collections::BTreeMap, fs::File};
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


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn to_tuple(&self) -> (usize,usize) {
        (self.x, self.y)
    }
}


pub struct PointList {
    points: Vec<Point>,
}


impl PointList {
    pub fn get_start(&self) -> Point {
        *self.points.first().unwrap()
    }

    pub fn get_end(&self) -> Point {
        *self.points.last().unwrap()
    }
}


#[derive(Clone, Copy, Debug)]
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

    fn get_connections(&self) -> Option<Vec<&Point>> {
        let mut connections: Vec<&Point> = Vec::with_capacity(4); // cardinal directions
        if let Some(node) = &self.conections.north {
            connections.push(node);
        }
        if let Some(node) = &self.conections.east {
            connections.push(node);
        }
        if let Some(node) = &self.conections.south {
            connections.push(node);
        }
        if let Some(node) = &self.conections.west {
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

    pub fn get_point_connections(&self, point: &Point) -> Option<Vec<&Point>> {
        if let Some(point) = self.nodes.get(point){
            point.get_connections()
        }
        else { None }
    }

    pub fn get_dimensions(&self) -> Dimensions {
        self.dimensions
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
    pub fn find_start(&self) -> (Point,Point) {
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
        (entrace_list[0],entrace_list[1])
    }

    pub fn solve_maze(&self, start: &Point, end: &Point) -> Path {
        // let mut _path: Vec<Point> = Vec::new();
        // let _last = start;

        // let path = self._recurse_solve(start, end, _last);
        let path = self.iter_solve(start, end);

        path
    }

    fn _recurse_solve(&self, start: &Point, end: &Point, last: &Point) {
        if start == end {
            println!("Found exit: {end:?}");
            exit(0)
        }
    
        let current_node = self.nodes.get(start).unwrap();
        if let Some(connection_points) = current_node.get_connections() {
            for point in connection_points.iter() {
                println!("point: {point:?}  start: {start:?}");
                if *point == last { continue; }
    
                let last = start;
                self._recurse_solve( point, end, last)
            }
        }
    }

    fn iter_solve(&self, start: &Point, end: &Point) -> Path {
        let mut node = self.nodes.get(start).unwrap();
        let mut visited_nodes: Vec<&Point> = Vec::new();
        visited_nodes.push(start);
        let mut path = Path::new();

        loop {
            let connections = node.get_connections().unwrap();
            let next_point = connections.iter().find(|x| !visited(*x, &visited_nodes) );

            if let Some(point) = next_point {
                path.push(point);
                visited_nodes.push(point);

                node = self.nodes.get(point).unwrap();
                if *point == end { break }
            }
            else {
                node = self.nodes.get(&path.pop()).unwrap();
            }
        }

        path
    }

    fn get_from_bool(x: bool) -> u8 {
        match x {
            true => 1,
            false => 0,
        }
    }
}

fn visited( point: &Point, visited: &Vec<&Point>) -> bool {
    if let Some(_) = visited.iter().find(|x| **x == point ) {
        true
    } else {
        false
    }
}

#[derive(Clone)]
pub struct Path<'a> {
    path: Vec<&'a Point>,
}

impl<'a> Path<'a> {
    fn new() -> Self { 
        Path { path: Vec::new() }
    }

    pub fn head(&self) -> &Point {
        self.path[0]
    }

    pub fn next(&self, index: usize) -> Option<&'a Point> {
        if self.path.len() > index {
            return Some(self.path[index])
        }
        None
    }    

    fn push(&mut self, point: &'a Point) {
        self.path.push(point);
    }

    fn pop(&mut self) -> &Point {
        self.path.pop().unwrap()
    }

    fn _print(&self) {
        for node in &self.path {
            print!("{node:?} ");
        }
        println!();
    }

    pub fn into_iter(&self) -> std::slice::Iter<'_, &Point>  {
         self.path.iter()
    }
}

pub struct Pnger {
    dimensions: Dimensions,
    bytes: Vec<u8>, 
}

impl Pnger {
    pub fn new(file_name: &str) -> Self {

        let open_file = File::open(file_name).unwrap_or_else(|e| {
            let error_string = format!("Error: {e:?}");
            panic!("urhg argh! {error_string}")
        });
        let decoder = png::Decoder::new(open_file);
        
        let mut reader = decoder.read_info().unwrap();
        let mut buffer = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buffer).unwrap();

        let bytes = &buffer[..info.buffer_size()];
        dbg!(info.buffer_size());
        let dimensions = Dimensions { height: info.height, width: info.width };

        Pnger { dimensions, bytes: bytes.to_vec() }
    }

    // pub fn get_maze_wall_size(&self) -> usize {
    //     let mut n: usize = 0;  // 
    //     while n < self.dimensions.height {
    //         if self.bytes[n] 
    //     }
    //     todo!();
    // }

    pub fn height(&self) -> u32 { self.dimensions.height }
    pub fn width(&self) -> u32 { self.dimensions.width }
    pub fn get_bytes(&self) -> Vec<u8> { self.bytes.clone() }
    pub fn dimensions(&self) -> Dimensions { self.dimensions }

    pub fn summarize(&self) {
        println!("width {} * height {}", self.width(), self.height());
    }
}