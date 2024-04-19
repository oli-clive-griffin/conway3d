use std::fmt::Display;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GameOfLife {
    width: usize,
    height: usize,
    depth: usize,
    revive: usize,
    lower: usize,
    upper: usize,
    cells: Vec<bool>,
}

#[wasm_bindgen]
impl GameOfLife {
    // pub fn new(width: usize, height: usize, depth: usize) -> GameOfLife {
    //     let cells = vec![false; width * height * depth];
    //     GameOfLife { width, height, depth, cells }
    // }

    // pub fn new_full(width: usize, height: usize, depth: usize) -> GameOfLife {
    //     let cells = vec![true; width * height * depth];
    //     GameOfLife { width, height, depth, cells }
    // }

    pub fn new_rand(
        width: usize,
        height: usize,
        depth: usize,
        revive: usize,
        lower: usize,
        upper: usize,
    ) -> GameOfLife {
        let mut cells = vec![false; width * height * depth];
        for i in 0..cells.len() {
            cells[i] = js_sys::Math::random() < 0.05;
        }
        GameOfLife {
            width,
            height,
            depth,
            cells,
            revive,
            lower,
            upper,
        }
    }

    pub fn new_stable() -> GameOfLife {
        let cells = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0,
            0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        GameOfLife {
            width: 5,
            height: 5,
            depth: 5,
            cells: cells.into_iter().map(|x| x == 1).collect(),
            revive: 4,
            lower: 5,
            upper: 6,
        }
    }

    pub fn new_stable2() -> GameOfLife {
        let cells = vec![
            0, 0, 0, 0, 0,
            0, 1, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,

            0, 0, 0, 0, 0,
            0, 0, 1, 0, 0,
            0, 1, 1, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,

            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,

            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,

            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,

        ];
        GameOfLife {
            width: 5,
            height: 5,
            depth: 5,
            cells: cells.into_iter().map(|x| x == 1).collect(),
            revive: 4,
            lower: 5,
            upper: 6,
        }
    }


    fn to_coords(&self, pos: usize) -> (usize, usize, usize) {
        let x = pos % self.width;
        let y = (pos / self.width) % self.height;
        let z = pos / (self.width * self.height);
        (z, y, x)
    }

    fn count(&self, z: usize, y: usize, x: usize) -> usize {
        let mut alive_neighbours = 0;
        for dz in 0..3 {
            let nz = ((((z as isize) + dz - 1) + (self.depth as isize)) % (self.depth as isize))
                as usize;
            for dy in 0..3 {
                let ny = ((((y as isize) + dy - 1) + (self.height as isize))
                    % (self.height as isize)) as usize;
                for dx in 0..3 {
                    let nx = ((((x as isize) + dx - 1) + (self.width as isize))
                        % (self.width as isize)) as usize;
                    let neighbour_index = nz * self.width * self.height + ny * self.width + nx;

                    if dz == 1 && dy == 1 && dx == 1 {
                        // println!("skipping index {} ", neighbour_index);
                        continue;
                    }
                    // println!("neighbour_index: {}", neighbour_index);

                    // alive_neighbours += unsafe { (self.cells.as_ptr().add(neighbour_index)) as u8; }
                    if self.cells[neighbour_index] {
                        alive_neighbours += 1;
                    };
                }
            }
        }
        alive_neighbours
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for i in 0..self.cells.len() {
            // let i = 0;
            let (z, y, x) = self.to_coords(i);
            // println!("z: {}, y: {}, x: {}", z, y, x);

            let alive_neighbours = self.count(z, y, x);

            // let next_cell = (!self.cells[i] && alive_neighbours == self.revive)
            //     || (self.lower <= alive_neighbours && alive_neighbours <= self.upper);

            let next_cell = if !self.cells[i] && alive_neighbours >= 14 && alive_neighbours <= 19 {
                true
            } else if alive_neighbours < 13 {
                false
            } else {
                self.cells[i]
            };

            if next_cell {
                println!(
                    "alive_neighbours: {}, {}",
                    alive_neighbours,
                    if self.cells[i] { "alive" } else { "dead" }
                );
            }
            next[i] = next_cell;
        }
        self.cells = next;
    }

    pub fn render(&self) -> Vec<u8> {
        // Convert the game state to a format that can be sent to JavaScript
        self.cells
            .iter()
            .map(|&cell| if cell { 1 } else { 0 })
            .collect()
    }
}

impl Display for GameOfLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for z in 0..self.depth {
            for y in 0..self.height {
                for x in 0..self.width {
                    let idx = z * self.width * self.height + y * self.width + x;
                    let cell = if self.cells[idx] { "X" } else { "." };
                    write!(f, " {} ", cell)?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::GameOfLife;

    #[test]
    fn idx() {
        let mut game = GameOfLife {
            cells: vec![false; 256],
            depth: 4,
            height: 4,
            width: 4,
            lower: 999,
            upper: 999,
            revive: 999,
        };
        assert_eq!(game.to_coords(0), (0, 0, 0));
        assert_eq!(game.to_coords(1), (0, 0, 1));
        assert_eq!(game.to_coords(4), (0, 1, 0));
        assert_eq!(game.to_coords(16), (1, 0, 0));
        assert_eq!(game.to_coords(17), (1, 0, 1));
        assert_eq!(game.to_coords(20), (1, 1, 0));
        game.tick();
    }

    #[test]
    fn asdf() {
        let game = GameOfLife {
            cells: vec![
                1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1,
                1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0,
                1, 0, 1, 1, 1, 0, 1, 1,
            ]
            .iter()
            .map(|i| *i == 1)
            .collect(),
            depth: 4,
            height: 4,
            width: 4,
            lower: 999,
            upper: 999,
            revive: 999,
        };
        let (z, y, x) = game.to_coords(15);
        assert_eq!(z, 0);
        assert_eq!(y, 3);
        assert_eq!(x, 3);
        assert_eq!(game.count(z, y, x), 26);
    }

    #[test]
    fn test_game() {
        let mut game = GameOfLife::new_stable2();
        println!("{}", game);
        println!("----------");
        game.tick();
        println!("{}", game);
        println!("----------");
        game.tick();
        println!("{}", game);
        println!("----------");
        game.tick();
        println!("{}", game);
        println!("----------");
        game.tick();
        println!("{}", game);
        println!("----------");
        game.tick();
        println!("{}", game);
        println!("----------");
        game.tick();
        println!("{}", game);
        println!("----------");
        game.tick();
        println!("{}", game);
        println!("----------");
        game.tick();

    }

    #[test]
    fn test_gameasdf() {
        let game = GameOfLife::new_stable();
        let (z, y, x) = game.to_coords(37);
        assert_eq!(z, 1);
        assert_eq!(y, 2);
        assert_eq!(x, 2);
        assert_eq!(game.count(z, y, x), 6)
    }
}
