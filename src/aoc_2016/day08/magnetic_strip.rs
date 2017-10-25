use aoc_2016::day08::Instruction;
use aoc_2016::day08::Instruction::*;

pub struct MagneticStrip {
    grid: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
}

impl MagneticStrip {
    pub fn new(row: usize, col: usize) -> MagneticStrip {
        MagneticStrip {
            grid: vec![vec![false; col]; row],
            rows: row,
            cols: col,
        }
    }

    pub fn execute(&mut self, instr: &Instruction) {
        match *instr {
            Rect(r, c) => self.rect(r, c),
            RotateRow(r, s) => self.rotate_row(r, s),
            RotateColumn(c, s) => self.rotate_column(c, s),
        }
    }

    pub fn run(&mut self, instructions: &[Instruction]) {
        instructions.iter().for_each(|instr| self.execute(instr));
    }

    fn rect(&mut self, cols: usize, rows: usize) {
        (0..rows).for_each(|row| (0..cols).for_each(|col| self.grid[row][col] = true));
    }

    fn rotate_row(&mut self, row: usize, shift: u32) {
        (0..shift).for_each(|_i| self.shift_row(row));
    }

    fn rotate_column(&mut self, col: usize, shift: u32) {
        (0..shift).for_each(|_i| self.shift_col(col));
    }

    fn shift_row(&mut self, row: usize) {
        let last_col = self.cols - 1;
        (0..last_col)
            .rev()
            .for_each(|col| self.swap_elements((row, col), (row, col + 1)));
    }

    fn shift_col(&mut self, col: usize) {
        let last_row = self.rows - 1;
        (0..last_row)
            .rev()
            .for_each(|row| self.swap_elements((row, col), (row + 1, col)));
    }

    fn swap_elements(&mut self, (r1, c1): (usize, usize), (r2, c2): (usize, usize)) {
        let tmp = self.grid[r1][c1];
        self.grid[r1][c1] = self.grid[r2][c2];
        self.grid[r2][c2] = tmp;
    }

    pub fn print(&self) {
        self.grid.iter().for_each(|row| {
            print!("| ");
            row.iter()
                .for_each(|e| print!("{}", MagneticStrip::symbol(e)));
            println!("");
        });
    }

    pub fn brightness(&self) -> usize {
        self.grid
            .iter()
            .fold(0, |b, row| b + row.iter().filter(|el| **el).count())
    }

    fn symbol(el: &bool) -> char {
        if *el {
            '#'
        } else {
            ' '
        }
    }
}
