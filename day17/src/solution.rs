pub struct ConwayHyperCubes {
    size: usize,
    board: Vec<Vec<Vec<Vec<bool>>>>
}

impl ConwayHyperCubes {
    pub fn from_lines(lines: impl Iterator<Item=String>) -> Self {
        let mut lines = lines.peekable();
        let first_line = lines.peek().unwrap();
        let size = if first_line.len() % 2 == 0 {
            first_line.len() + 13
        } else {
            first_line.len() + 12
        };
        let mut board = vec![vec![vec![vec![false; size]; size]; size]; size];
        for (y, line) in lines.enumerate() {
            for (x, cube) in line.chars().enumerate() {
                match cube {
                    '#' => board[size/2][size/2][y+6][x+6] = true,
                    '.' => (),
                    _ => panic!()
                }
            }
        }

        ConwayHyperCubes { size, board }
    }

    fn simulate_step(&mut self) {
        let mut toggle_list: Vec<(usize, usize, usize, usize)> = Vec::new();
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    for w in 0..self.size {
                        let active_neighbours =  self.active_neighbours(x, y, z, w);
                        if self.get(x, y, z, w).unwrap() {
                            if active_neighbours != 2 && active_neighbours != 3 {
                                toggle_list.push((x, y, z, w));
                            }
                        } else {
                            if active_neighbours == 3 {
                                toggle_list.push((x, y, z, w));
                            }
                        }
                    }
                }
            }
        }
        for &(x, y, z, w) in toggle_list.iter() {
            self.toggle(x, y, z, w);
        }
    }

    fn active_neighbours(&self, x: usize, y: usize, z: usize, w: usize) -> usize {
        let mut sum = 0;
        let mut shifts: Vec<(i32, i32, i32, i32)> = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    for w in -1..2 {
                        if x != 0 || y != 0 || z != 0 || w != 0 {
                            shifts.push((x, y, z, w));
                        }
                    }
                }
            }
        }
        for (shift_x, shift_y, shift_z, shift_w) in shifts.iter() {
            let shifted_x = (x as i32 + shift_x) as usize;
            let shifted_y = (y as i32 + shift_y) as usize;
            let shifted_z = (z as i32 + shift_z) as usize;
            let shifted_w = (w as i32 + shift_w) as usize;
            if let Some(neighbour) = self.get(shifted_x, shifted_y, shifted_z, shifted_w) {
                if neighbour {
                    sum += 1;
                }
            }
        }
        sum
    }

    fn get(&self, x: usize, y: usize, z: usize, w: usize) -> Option<bool> {
        if x < self.size && y < self.size && z < self.size && w < self.size {
            return Some(self.board[w][z][y][x]);
        }
        None
    }

    fn toggle(&mut self, x: usize, y: usize, z: usize, w: usize) {
        if x < self.size && y < self.size && z < self.size && w < self.size {
            self.board[w][z][y][x] = !self.board[w][z][y][x];
        }
    }

    fn active_cubes(&self) -> i32 {
        let mut actives = 0;
        for cube in &self.board {
            for plane in cube {
                for line in plane {
                    for el in line {
                        if *el {
                            actives += 1;
                        }
                    }
                }
            }
        }
        actives
    }
}

pub struct ConwayCubes {
    size: usize,
    board: Vec<Vec<Vec<bool>>>
}

impl ConwayCubes {
    pub fn from_lines(lines: impl Iterator<Item=String>) -> Self {
        let mut lines = lines.peekable();
        let first_line = lines.peek().unwrap();
        let size = first_line.len() + 12;
        let plane = vec![vec![false; size]; size];
        let mut board: Vec<Vec<Vec<bool>>> = Vec::new();
        for _ in 0..(size / 2) {
            board.push(plane.clone());
        }
        let mut initial_plane: Vec<Vec<bool>> = Vec::new();
        for _ in 0..6 {
            initial_plane.push(vec![false; size]);
        }
        for line in lines {
            let mut cube_line = vec![false; 6];
            for c in line.chars() {
                match c {
                    '#' => cube_line.push(true),
                    '.' => cube_line.push(false),
                    _ => panic!()
                }
            }
            cube_line.append(&mut vec![false; 6]);
            initial_plane.push(cube_line);
        }
        for _ in 0..6 {
            initial_plane.push(vec![false; size]);
        }
        board.push(initial_plane);

        for _ in 0..(size / 2) {
            board.push(plane.clone());
        }
        for layer in &board {
            dbg!(layer.len());
        }
        ConwayCubes { size, board }
    }

    fn simulate_step(&mut self) {
        let mut toggle_list: Vec<(usize, usize, usize)> = Vec::new();
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    let active_neighbours =  self.active_neighbours(x, y, z);
                    if self.get(x, y, z).unwrap() {
                        if active_neighbours != 2 && active_neighbours != 3 {
                            toggle_list.push((x, y, z));
                        }
                    } else {
                        if active_neighbours == 3 {
                            toggle_list.push((x, y, z));
                        }
                    }
                }
            }
        }
        for &(x, y, z) in toggle_list.iter() {
            self.toggle(x, y, z);
        }
    }

    fn active_neighbours(&self, x: usize, y: usize, z: usize) -> usize {
        let mut sum = 0;
        let shifts = vec![
            (-1, -1, -1),
            (-1, -1, 0),
            (-1, -1, 1),
            (-1, 0, -1),
            (-1, 0, 0),
            (-1, 0, 1),
            (-1, 1, -1),
            (-1, 1, 0),
            (-1, 1, 1),
            (1, -1, -1),
            (1, -1, 0),
            (1, -1, 1),
            (1, 0, -1),
            (1, 0, 0),
            (1, 0, 1),
            (1, 1, -1),
            (1, 1, 0),
            (1, 1, 1),
            (0, -1, -1),
            (0, -1, 0),
            (0, -1, 1),
            (0, 0, -1),
            (0, 0, 1),
            (0, 1, -1),
            (0, 1, 0),
            (0, 1, 1),
        ];
        for (shift_x, shift_y, shift_z) in shifts.iter() {
            let shifted_x = (x as i32 + shift_x) as usize;
            let shifted_y = (y as i32 + shift_y) as usize;
            let shifted_z = (z as i32 + shift_z) as usize;
            if let Some(neighbour) = self.get(shifted_x, shifted_y, shifted_z) {
                if neighbour {
                    sum += 1;
                }
            }
        }
        sum
    }

    fn get(&self, x: usize, y: usize, z: usize) -> Option<bool> {
        if x < self.size && y < self.size && z < self.size {
            return Some(self.board[z][y][x]);
        }
        None
    }

    fn toggle(&mut self, x: usize, y: usize, z: usize) {
        if x < self.size && y < self.size && z < self.size {
            self.board[z][y][x] = !self.board[z][y][x];
        }
    }

    fn active_cubes(&self) -> i32 {
        let mut actives = 0;
        for plane in &self.board {
            for line in plane {
                for el in line {
                    if *el {
                        actives += 1;
                    }
                }
            }
        }
        actives
    }
}

pub fn part1(mut cube: ConwayCubes) -> i32 {
    for _ in 0..6 {
        cube.simulate_step();
    }
    cube.active_cubes()
}

pub fn part2(mut hyper_cube: ConwayHyperCubes) -> i32 {
    for _ in 0..6 {
        hyper_cube.simulate_step();
    }
    hyper_cube.active_cubes()
}

mod test {
    #[test]
    fn example_part1() {
        let input = vec![String::from(".#."), String::from("..#"), String::from("###")];
        let mut cube = super::ConwayCubes::from_lines(input.into_iter());
        for _ in 0..6 {
            cube.simulate_step();
        }
        assert_eq!(cube.active_cubes(), 112);
    }

    #[test]
    fn example_part2() {
        let input = vec![String::from(".#."), String::from("..#"), String::from("###")];
        let mut cube = super::ConwayHyperCubes::from_lines(input.into_iter());
        for _ in 0..6 {
            cube.simulate_step();
        }
        assert_eq!(cube.active_cubes(), 848);
    }
}
