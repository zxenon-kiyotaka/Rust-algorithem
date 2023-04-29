use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let width = 10;
    let height = 10;
    let mut maze = Maze::new(width, height);
    maze.generate(&mut rng);
    println!("{}", maze);
}

struct Maze {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        let mut cells = Vec::new();
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                row.push(Cell::new(x, y));
            }
            cells.push(row);
        }
        Maze {
            width,
            height,
            cells,
        }
    }

    fn generate(&mut self, rng: &mut impl Rng) {
        let mut stack = Vec::new();
        let mut current = &mut self.cells[0][0];
        current.visited = true;
        stack.push((0, 0));
        while let Some((x, y)) = stack.pop() {
            let neighbors = self.get_unvisited_neighbors(x, y);
            if let Some((nx, ny)) = rng.choose(&neighbors) {
                let next = &mut self.cells[*ny][*nx];
                current.remove_wall(*nx, *ny);
                next.visited = true;
                stack.push((x, y));
                stack.push((*nx, *ny));
                current = next;
            } else if let Some((px, py)) = stack.last() {
                current = &mut self.cells[*py][*px];
            }
        }
    }

    fn get_unvisited_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        if x > 0 && !self.cells[y][x - 1].visited {
            neighbors.push((x - 1, y));
        }
        if y > 0 && !self.cells[y - 1][x].visited {
            neighbors.push((x, y - 1));
        }
        if x < self.width - 1 && !self.cells[y][x + 1].visited {
            neighbors.push((x + 1, y));
        }
        if y < self.height - 1 && !self.cells[y + 1][x].visited {
            neighbors.push((x, y + 1));
        }
        neighbors
    }
}

impl std::fmt::Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.cells[y][x];
                output.push(if cell.walls[0] { '#' } else { ' ' });
                output.push(if cell.walls[1] { '#' } else { ' ' });
            }
            output.push('\n');
            for x in 0..self.width {
                let cell = &self.cells[y][x];
                output.push(if cell.walls[2] { '#' } else { ' ' });
                output.push(if cell.walls[3] { '#' } else { ' ' });
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

struct Cell {
    x: usize,
    y: usize,
    walls: [bool; 4],
    visited: bool,
}

impl Cell {
    fn new(x: usize, y: usize) -> Self {
        Cell {
            x,
            y,
            walls: [true; 4],
            visited: false,
        }
    }

    fn remove_wall(&mut self, x: usize, y: usize) {
        if self.x == x && self.y > y {
            self.walls[1] = false;
        } else if self.x == x && self.y < y {
            self.walls[3] = false;
        } else if self.x > x && self.y == y {
            self.walls[0] = false;
        } else if self.x < x && self.y == y {
            self.walls[2] = false;
        }
    }
}

