use maze_walker::*;

fn main() {
    let image = Pnger::new("mazes/maze(9).png");
    let pixel_list = PixelList::new(&image.get_bytes(), image.dimensions());

    let maze = Maze::new(
        image.dimensions(),
        &pixel_list,
    );

    maze.print_maze();

    let entrances = maze.find_start();
    println!("Entrances {:?} {:?}", entrances.get_start(), entrances.get_end());
    maze.solve_maze( &entrances.get_start(), &entrances.get_end());
}