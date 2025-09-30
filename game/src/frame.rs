pub struct Frame {
    pub width: usize,
    pub height: usize,
    _frame: [[char; 110]; 40],
}

impl Frame {
    pub fn new() -> Self {
        return Self {
            width: 110,
            height: 40,
            _frame: [(); 40].map(|_| [' '; 110])
        };
    }

    pub fn box(&mut self, x: usize, y: usize, w: usize, h: usize, ch: char) {
        let mut i = x;
        let mut j = y;
        let width = x + w;
        let height = y + h;
        // TODO look up actual syntax later
        while i < width {
            self.write(i, y, ch);
            self.write(i, height, ch);

            i = i + 1;
        }

        while j < height {
            self.write(x, j, ch);
            self.write(width, j, ch);

            j = j + 1;
        }
    }

    pub fn hline(&mut self, x: usize, y: usize, l: usize, ch: char) {
        self.box(x, y, l, 1, ch);
    }

    pub fn wline(&mut self, x: usize, y:usize, l: usize, ch: char) {
        self.box(x, y, 1, l, ch);
    }

    pub fn text(&mut self, x: usize, y: usize, s: &str) {
        let mut i = 0;
        for ch in s.chars() {
            // if x + i < self._frame[0].len() && y < self._frame.len() {
            //     self._frame[y][x + i] = ch;
            // }
            self.write(x + i, y, ch);

            i = i + 1;
        }
    }

    // this is probably horribly inefficient
    // but can be copy/pasted later
    pub fn write(&mut self, x: usize, y: usize, ch: char) {
        if x < self.width && y < self.height {
            self._frame[y][x] = ch;
        }
    }
}