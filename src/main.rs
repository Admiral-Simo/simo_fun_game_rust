use rand::random;
use std::io::{self, Write};

#[derive(Clone, Debug, PartialEq, Copy)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
}

impl Color {
    fn as_str(&self) -> &'static str {
        match self {
            Color::Red => "\x1b[31mR\x1b[0m",
            Color::Green => "\x1b[32mG\x1b[0m",
            Color::Blue => "\x1b[34mB\x1b[0m",
            Color::Yellow => "\x1b[33mY\x1b[0m",
            Color::Cyan => "\x1b[36mC\x1b[0m",
            Color::Magenta => "\x1b[35mM\x1b[0m",
        }
    }
}

fn initialize_grid(grid: &mut Vec<Vec<Color>>, size: u8) {
    for _ in 0..size {
        let mut tmp = vec![];
        for _ in 0..size {
            let random_color = match random::<u32>() % 6 {
                0 => Color::Red,
                1 => Color::Green,
                2 => Color::Blue,
                3 => Color::Yellow,
                4 => Color::Cyan,
                _ => Color::Magenta,
            };
            tmp.push(random_color);
        }
        grid.push(tmp);
    }
}

fn print_grid(grid: &Vec<Vec<Color>>) {
    for row in grid {
        for cell in row {
            print!("{} ", cell.as_str())
        }
        println!();
    }
}

fn switch_iceland_color(grid: &mut Vec<Vec<Color>>, color: Color) {
    if grid.is_empty() {
        return;
    }
    let start_color = grid[0][0];
    if start_color == color {
        return;
    }
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    dfs(grid, color, &mut visited, 0, 0, start_color);
}

fn dfs(
    grid: &mut Vec<Vec<Color>>,
    color: Color,
    visited: &mut Vec<Vec<bool>>,
    row: usize,
    col: usize,
    start_color: Color,
) {
    if row >= grid.len()
        || col >= grid[0].len()
        || grid[row][col] != start_color
        || visited[row][col]
    {
        return;
    }
    grid[row][col] = color;
    visited[row][col] = true;
    if row + 1 < grid.len() {
        dfs(grid, color, visited, row + 1, col, start_color);
    }
    if row > 0 {
        dfs(grid, color, visited, row - 1, col, start_color);
    }
    if col + 1 < grid[0].len() {
        dfs(grid, color, visited, row, col + 1, start_color);
    }
    if col > 0 {
        dfs(grid, color, visited, row, col - 1, start_color);
    }
}

fn is_grid_solved(grid: &Vec<Vec<Color>>) -> bool {
    if let Some(first_row) = grid.first() {
        if let Some(initial_color) = first_row.first() {
            for row in grid {
                for color in row {
                    if *color != *initial_color {
                        return false;
                    }
                }
            }
            return true;
        }
    }
    true
}

fn main() {
    // Initialize the grid
    let mut grid = vec![];
    // chose the difficulty
    print!("Chose a difficulty (easy, medium, hard): ");
    let mut input = String::new();

    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let size: u8 = match input.trim().to_lowercase().as_str() {
        "easy" => 10,
        "medium" => 20,
        "hard" => 40,
        _ => {
            println!("Invalid difficulty. Defaulting to medium.");
            20
        }
    };
    initialize_grid(&mut grid, size);

    clearscreen::clear().expect("failed to clear screen");

    // Print the grid
    println!("Grid:");
    print_grid(&grid);

    const MAX_TRIES: u8 = 90;
    let mut tries: u8 = MAX_TRIES;
    // Prompt the user to input the desired color
    while !is_grid_solved(&grid) && tries > 0 {
        println!("You have {tries} tries choose a color to switch the island (Red, Green, Blue, Yellow, Cyan, Magenta): ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let chosen_color = match input.trim().to_lowercase().as_str() {
            "r" => Color::Red,
            "g" => Color::Green,
            "b" => Color::Blue,
            "y" => Color::Yellow,
            "c" => Color::Cyan,
            "m" => Color::Magenta,
            _ => {
                println!("Invalid color. Defaulting to Green.");
                Color::Green
            }
        };
        clearscreen::clear().expect("failed to clear screen");

        // Switch colors starting from index (0, 0) and infecting all the island
        switch_iceland_color(&mut grid, chosen_color);

        // Print the grid after the color switch
        println!("Grid :");
        print_grid(&grid);
        tries -= 1;
    }

    if is_grid_solved(&grid) {
        println!("Congrats you won the game :))))))))))).");
    } else {
        println!("You are left with {tries} out of {MAX_TRIES} you failed the game try again.");
    }
}
