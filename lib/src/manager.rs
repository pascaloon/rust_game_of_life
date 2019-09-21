use std::fs;
use std::error::Error;


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CellState {
    Dead,
    Alive
}

pub struct Map {
    pub states: Vec<CellState>,

    width: usize,
    height: usize,
}

impl Map {
    pub fn get_cell(&self, x: usize, y: usize) -> & CellState {
        & self.states[(y * self.width) + x]
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> &mut CellState {
        &mut self.states[(y * self.width) + x]
    }

    pub fn get_width(&self) -> usize { self.width }
    pub fn get_height(&self) -> usize { self.height }
}

pub struct Game {
    pub map: Map
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        let states = vec![CellState::Dead; (width * height) as usize];
        let map = Map {states: states, width: width, height: height};
        Game {map: map}
    }

    pub fn run(&mut self, count: usize) {
        for _ in 0..count {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let mut new_states = self.map.states.clone();
        for y in 1..self.map.height-1 {
            for x in 1..self.map.width-1 {

                // Count all alive cells in surrounding block
                let mut alive_friends = 0;
                for y1 in (y-1)..(y+2) {
                    for x1 in (x-1)..(x+2) {
                        match self.map.states[(y1 * self.map.width) + x1] {
                            CellState::Alive => alive_friends += 1,
                            _ => ()
                        }
                    }
                }

                let cell_offset = (y * self.map.width) + x;
                let cell = & self.map.states[cell_offset];
                let new_cell = &mut new_states[cell_offset];
                
                // Adjust alive count if the cell itself was alive
                alive_friends = match cell {
                    CellState::Alive => alive_friends - 1,
                    CellState::Dead => alive_friends
                };
                
                // Rules
                // A live cell that has fewer than two live neighbors dies from loneliness.
                // A live cell that has more than three live neighbors dies from overcrowding.
                // A live cell that has two or three live neighbors stays alive.
                // A dead cell that has exactly three live neighbors becomes alive.

                *new_cell = match cell {
                    CellState::Alive => match alive_friends {
                        0 | 1 => CellState::Dead,
                        2 | 3 => CellState::Alive,
                        _ => CellState::Dead
                    },
                    CellState::Dead => match alive_friends {
                        3 => CellState::Alive,
                        _ => CellState::Dead
                    }
                }

            } // for x
        } // for y
        self.map.states = new_states;
    }

    pub fn get_states_as_string(&self) -> String {
        // let mut result = String::with_capacity(self.map.width * self.map.height + (self.map.height - 1));
        let mut result = Vec::<char>::with_capacity(self.map.width * self.map.height + (self.map.height - 1));

        for y in 0..self.map.height {
            for x in 0..self.map.width {
                result.push(match self.map.get_cell(x, y) {
                    CellState::Alive => 'x',
                    CellState::Dead => '.'
                });
            }
            result.push('\n');
        }

        result.resize(result.len() - 1, 'E');
        result.into_iter().collect()
    }
}

pub fn load_game_from_file(filename: &String) -> Result<Game, Box<dyn Error>> {
    let content = fs::read_to_string(& filename)?;
    load_from_string(& content)

}

pub fn load_from_string(content: &String) -> Result<Game, Box<dyn Error>> {
    let mut lines = content.lines();
    let first_line = lines.next();
    if first_line.is_none() {
        panic!("Empty file");
    }
    let first_line : &str = first_line.unwrap();
    let first_line_parts: Vec<&str> = first_line.split('x').collect();
    let width = first_line_parts[0].parse::<usize>()?;
    let height = first_line_parts[1].parse::<usize>()?;

    load_from_stringlines(&mut lines, width, height)
}

pub fn load_from_stringmap(content: &String, width: usize, height: usize) -> Result<Game, Box<dyn Error>> {
    let mut lines = content.lines();
    load_from_stringlines(&mut lines, width, height)
}

fn load_from_stringlines(lines: &mut std::str::Lines, width: usize, height: usize) -> Result<Game, Box<dyn Error>> {
    // println!("Parsing map size {}x{}", width, height);

    let mut game = Game::new(width, height);

    let mut y: usize = 0;
    while let Some(line) = lines.next() {
        assert_eq!(line.len(), width);
        // println!("{}", line);
        let mut x: usize = 0;
        for c in line.chars() {
            game.map.states[(width * y) + x] = match c {
                'x' => CellState::Alive,
                '.' => CellState::Dead,
                _ => panic!("Unknown character '{}' in map. Position ({}, {})", c, x, y)
            };
            x += 1;
        }
        y += 1;
    }
    assert_eq!(y, height);

    Ok(game)
}

pub fn print_game(game: &Game) {
    for y in 1..game.map.height-1 {
        for x in 1..game.map.width-1 {
            let cell = & game.map.states[(y * game.map.width) + x];
            match cell {
                CellState::Alive => print!("x"),
                CellState::Dead => print!(".")
            };
        }
        println!("");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_get_cell() {
        let width = 3;
        let height = 4;

        let map = Map {states: vec![CellState::Dead; 12], width: width, height: height};

        let cell = map.get_cell(1, 3);
        let cell2 = & map.states[(3 * width) + 1];

        assert_eq!(cell as *const _, cell2 as *const _);
    }

    #[test]
    fn test_map_get_cell_mut() {
        let width = 3;
        let height = 4;

        let mut map = Map {states: vec![CellState::Dead; 12], width: width, height: height};

        let cell = map.get_cell_mut(1, 3) as *const _;
        let cell2 = & map.states[(3 * width) + 1] as *const _;

        assert_eq!(cell, cell2);
    }

    #[test]
    fn test_load_content() {
        let game = load_from_string(& String::from("3x3\nx..\n.x.\n..x")).expect("Couldn't parse string");

        assert_eq!(3, game.map.width);
        assert_eq!(3, game.map.height);
        
        let cells = &game.map.states;
        
        assert_eq!(CellState::Alive, cells[(0 * game.map.width) + 0]);
        assert_eq!(CellState::Dead, cells[(0 * game.map.width) + 1]);
        assert_eq!(CellState::Dead, cells[(0 * game.map.width) + 2]);

        assert_eq!(CellState::Dead, cells[(1 * game.map.width) + 0]);
        assert_eq!(CellState::Alive, cells[(1 * game.map.width) + 1]);
        assert_eq!(CellState::Dead, cells[(1 * game.map.width) + 2]);

        assert_eq!(CellState::Dead, cells[(2 * game.map.width) + 0]);
        assert_eq!(CellState::Dead, cells[(2 * game.map.width) + 1]);
        assert_eq!(CellState::Alive, cells[(2 * game.map.width) + 2]);
    }

    #[test]
    fn test_load_stringmap() {
        let game = load_from_stringmap(& String::from("x..\n.x.\n..x"), 3, 3).expect("Couldn't parse string");

        assert_eq!(3, game.map.width);
        assert_eq!(3, game.map.height);
        
        let cells = &game.map.states;
        
        assert_eq!(CellState::Alive, cells[(0 * game.map.width) + 0]);
        assert_eq!(CellState::Dead, cells[(0 * game.map.width) + 1]);
        assert_eq!(CellState::Dead, cells[(0 * game.map.width) + 2]);

        assert_eq!(CellState::Dead, cells[(1 * game.map.width) + 0]);
        assert_eq!(CellState::Alive, cells[(1 * game.map.width) + 1]);
        assert_eq!(CellState::Dead, cells[(1 * game.map.width) + 2]);

        assert_eq!(CellState::Dead, cells[(2 * game.map.width) + 0]);
        assert_eq!(CellState::Dead, cells[(2 * game.map.width) + 1]);
        assert_eq!(CellState::Alive, cells[(2 * game.map.width) + 2]);
    }

    fn test_input(width: usize, height: usize, map: &String, expected: &String, steps: usize) {
        let mut game = load_from_stringmap(& map, width, height).expect("Couldn't parse string");
        game.run(steps);

        let result = game.get_states_as_string();
        println!("map:\n{}\n", map);
        println!("expected:\n{}\n", expected);
        println!("result\n{}\n", result);
        assert_eq!(*expected, result);
    }

    #[test]
    fn test_cross() {
        let map = String::from("\
.....
..x..
..x..
..x..
.....");
        let expected = String::from("\
.....
.....
.xxx.
.....
.....");
        test_input(5, 5, &map, &expected, 5);
        test_input(5, 5, &map, &map, 6);
    }
}