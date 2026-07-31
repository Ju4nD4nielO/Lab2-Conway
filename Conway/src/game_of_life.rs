pub struct GameOfLife {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<bool>>,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        GameOfLife {
            width,
            height,
            cells: vec![vec![false; width]; height],
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, alive: bool) {
        if x < self.width && y < self.height {
            self.cells[y][x] = alive;
        }
    }

    pub fn set_pattern(&mut self, x: usize, y: usize, pattern: &[&str]) {
        for (dy, row) in pattern.iter().enumerate() {
            for (dx, cell) in row.chars().enumerate() {
                if cell == '#' {
                    self.set_cell(x + dx, y + dy, true);
                }
            }
        }
    }

    pub fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0
                    && nx < self.width as i32
                    && ny >= 0
                    && ny < self.height as i32
                {
                    if self.cells[ny as usize][nx as usize] {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    pub fn next_generation(&mut self) {
        let mut next = vec![vec![false; self.width]; self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_neighbors(x, y);

                if self.cells[y][x] {
                    next[y][x] = neighbors == 2 || neighbors == 3;
                } else {
                    next[y][x] = neighbors == 3;
                }
            }
        }

        self.cells = next;
    }

    // =========================
    // STILL LIFES
    // =========================

    pub fn block(&mut self, x: usize, y: usize) {
        let pattern = [
            "##",
            "##",
        ];

        self.set_pattern(x, y, &pattern);
    }

    pub fn beehive(&mut self, x: usize, y: usize) {
        let pattern = [
            ".##.",
            "#..#",
            ".##.",
        ];

        self.set_pattern(x, y, &pattern);
    }

    pub fn loaf(&mut self, x: usize, y: usize) {
        let pattern = [
            ".##.",
            "#..#",
            ".#.#",
            "..#.",
        ];

        self.set_pattern(x, y, &pattern);
    }

    pub fn boat(&mut self, x: usize, y: usize) {
        let pattern = [
            "##.",
            "#.#",
            ".#.",
        ];

        self.set_pattern(x, y, &pattern);
    }

    pub fn tub(&mut self, x: usize, y: usize) {
        let pattern = [
            ".#.",
            "#.#",
            ".#.",
        ];

        self.set_pattern(x, y, &pattern);
    }

    // =========================
// OSCILLATORS
// =========================

pub fn blinker(&mut self, x: usize, y: usize) {
    let pattern = [
        "###",
    ];

    self.set_pattern(x, y, &pattern);
}

pub fn toad(&mut self, x: usize, y: usize) {
    let pattern = [
        ".###",
        "###.",
    ];

    self.set_pattern(x, y, &pattern);
}

pub fn beacon(&mut self, x: usize, y: usize) {
    let pattern = [
        "##..",
        "##..",
        "..##",
        "..##",
    ];

    self.set_pattern(x, y, &pattern);
}

pub fn pulsar(&mut self, x: usize, y: usize) {
    let pattern = [
        "..###...###..",
        ".............",
        "#....#.#....#",
        "#....#.#....#",
        "#....#.#....#",
        "..###...###..",
        ".............",
        "..###...###..",
        "#....#.#....#",
        "#....#.#....#",
        "#....#.#....#",
        ".............",
        "..###...###..",
    ];

    self.set_pattern(x, y, &pattern);
}

pub fn pentadecathlon(&mut self, x: usize, y: usize) {
    let pattern = [
        "...#...",
        "...#...",
        "..#.#..",
        "...#...",
        "...#...",
        "...#...",
        "..#.#..",
        "...#...",
        "...#...",
        "...#...",
        "..#.#..",
        "...#...",
        "...#...",
    ];

    self.set_pattern(x, y, &pattern);
}

// =========================
// SPACESHIPS
// =========================

pub fn glider(&mut self, x: usize, y: usize) {
    let pattern = [
        ".#.",
        "..#",
        "###",
    ];

    self.set_pattern(x, y, &pattern);
}

pub fn lwss(&mut self, x: usize, y: usize) {
    let pattern = [
        ".#..#",
        "#....",
        "#...#",
        "####.",
    ];

    self.set_pattern(x, y, &pattern);
}

pub fn mwss(&mut self, x: usize, y: usize) {
    let pattern = [
        "..#...",
        "#...#.",
        ".....#",
        "#....#",
        "######",
    ];

    self.set_pattern(x, y, &pattern);
}

pub fn hwss(&mut self, x: usize, y: usize) {
    let pattern = [
        "..##...",
        "#....#.",
        "......#",
        "#.....#",
        "#######",
    ];

    self.set_pattern(x, y, &pattern);
}

// =========================
// ESCENA FINAL
// =========================

pub fn create_final_scene(&mut self) {
    // -------------------------
    // STILL LIFES
    // -------------------------

    self.block(3, 3);
    self.beehive(12, 3);
    self.loaf(23, 3);
    self.boat(34, 3);
    self.tub(43, 3);

    // -------------------------
    // OSCILLATORS
    // -------------------------

    self.blinker(55, 5);
    self.toad(65, 4);
    self.beacon(82, 4);

    self.pulsar(8, 18);
    self.pentadecathlon(40, 20);

    // -------------------------
    // SPACESHIPS
    // -------------------------

    // Gliders
    self.glider(70, 20);
    self.glider(85, 30);
    self.glider(55, 45);

    // Lightweight Spaceship
    self.lwss(10, 45);

    // Middleweight Spaceship
    self.mwss(32, 48);

    // Heavyweight Spaceship
    self.hwss(65, 50);

    // Otro Glider en la parte inferior
    self.glider(82, 65);
}

}