// const FRAME_WIDTH: usize = 110;
// const FRAME_HEIGHT: usize = 40;
const FRAME_WIDTH: usize = 80;
const FRAME_HEIGHT: usize = 30;

pub struct Frame {
    pub width: usize,
    pub height: usize,
    _frame: [[char; FRAME_WIDTH]; FRAME_HEIGHT],
}

impl Frame {
    pub fn new() -> Self {
        return Self {
            width: FRAME_WIDTH,
            height: FRAME_HEIGHT,
            _frame: [(); FRAME_HEIGHT].map(|_| [' '; FRAME_WIDTH])
        };
    }

	pub fn clear_line(&mut self, y: usize) {
		self.line_horizontal(0, y, FRAME_WIDTH, ' ');
	}

	pub fn each_line(&mut self) -> std::slice::Iter<'_, [char; FRAME_WIDTH]> {
		self._frame.iter()
	}

    pub fn line_horizontal(&mut self, x: usize, y: usize, l: usize, ch: char) {
        self.rect(x, y, l, 1, ch);
    }

    pub fn line_vertical(&mut self, x: usize, y:usize, l: usize, ch: char) {
        self.rect(x, y, 1, l, ch);
    }

    pub fn rect(&mut self, x: usize, y: usize, w: usize, h: usize, ch: char) {
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

	pub fn split_line(&self, x_offset: usize, line: &str) -> Vec<String> {
		let mut x = x_offset;
		let mut strs:Vec<String> = Vec::new();
		let mut current = String::new();
		for sub_line in line.split('\n') {
			for s in sub_line.split(' ') {
				if x + s.len() >= FRAME_WIDTH {
					strs.push(current.clone());
					current = String::new();
				}

				current = current + s + " ";
			}
			strs.push(current.clone());
			current = String::new();
		}

		return strs;
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
