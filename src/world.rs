pub struct World {
    width: usize,
    height: usize,
    state: Box<[Particle]>,
    next_state: Box<[Particle]>
}

impl World {
    pub fn new(width: usize, height: usize, grid_a: Box<[Particle]>, grid_b: Box<[Particle]>) -> World {
        World {
            width,
            height,
            state: grid_a,
            next_state: grid_b
        }
    }

    pub fn draw(&self, frame: &mut [u8], selected_particle: Particle) {
        for y in 0..self.height {
            let w = self.width + 3;
            for x in 0..self.width {
                let c = self.state[x + y*self.width].color();
                frame[4*(x + y*w) + 0] = c.0;
                frame[4*(x + y*w) + 1] = c.1;
                frame[4*(x + y*w) + 2] = c.2;
            }
            let p = Particle::from_rank(y);
            let c = p.color();
            frame[4*(y+1)*w - 4] = c.0;
            frame[4*(y+1)*w - 3] = c.1;
            frame[4*(y+1)*w - 2] = c.2;
            // Selected particle type in red
            frame[4*(y+1)*w - 8] = if p == selected_particle { 255 } else { 0 };
        }
    }

    pub fn spawn(&mut self, x: usize, y: usize, t: Particle) {
        self.next_state[x + y*self.width] = t;
    }

    pub fn update(&mut self) {
        self.update_buffers();
        for y in (0..self.height).rev() {
            for x in (0..self.width).rev() {
                match self.state[x + y*self.width] {
                    Particle::Sand  => self.update_sand(x, y),
                    Particle::Stone => self.update_stone(x, y),
                    Particle::Water => self.update_water(x, y),
                    Particle::Oil   => self.update_oil(x, y),
                    Particle::Void  => ()
                }
            }
        }
    }

    fn update_buffers(&mut self) {
        for (p, new_p) in self.state.iter_mut().zip(self.next_state.iter()) {
            *p = *new_p;
        }
    }

    fn swap(&mut self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) {
        let temp = self.state[x1 + y1*self.width];
        self.next_state[x1 + y1*self.width] = self.next_state[x2 + y2*self.width];
        self.next_state[x2 + y2*self.width] = temp;
    }

    fn update_sand(&mut self, x: usize, y: usize) {
        if y == self.height-1 { return }

        // No match because sand behaves the same way with every particle
        let d = self.state[x + y*self.width].density();

        if d > self.next_state[x + (y+1)*self.width].density() {
            self.swap((x, y), (x, y+1));
        }
        else {
            if x != 0 && d > self.next_state[(x-1) + y*self.width].density() && d > self.next_state[(x-1) + (y+1)*self.width].density() {
                self.swap((x, y), (x-1, y+1))
            }
            else if x != self.width-1 && d > self.next_state[(x+1) + y*self.width].density() && d > self.next_state[(x+1) + (y+1)*self.width].density() {
                self.swap((x, y), (x+1, y+1))
            }
            else {
                self.next_state[x + y*self.width] = Particle::Sand;
            }
        }
    }

    fn update_stone(&mut self, x: usize, y: usize) {
        self.next_state[x + y*self.width] = Particle::Stone;
    }

    fn update_water(&mut self, x: usize, y: usize) {
        self.liquid_behaviour(x, y);
    }

    fn update_oil(&mut self, x: usize, y: usize) {
        self.liquid_behaviour(x, y);
    }

    fn liquid_behaviour(&mut self, x: usize, y: usize) {
        if y == self.height-1 { return }

        let d = self.state[x + y*self.width].density();

        if d > self.next_state[x + (y+1)*self.width].density() {
            self.swap((x, y), (x, y+1));
        }
        else {
            // Flow direction
            let mut dir: Option<bool> = None;
            let mut loop_left = x != 0;
            let mut loop_right = x != self.width-1;

            for i in 1..usize::max(x, self.width-1 - x) {
                if loop_left {
                    if x - i == 0 { loop_left = false }
                    if d <= self.next_state[(x - i) + y*self.width].density() { loop_left = false }
                    else if d > self.next_state[(x - i) + (y+1)*self.width].density() {
                        dir = Some(false);
                        break;
                    }
                }
                if loop_right {
                    if x + i == self.width-1 { loop_right = false }
                    if d <= self.next_state[(x + i) + y*self.width].density() { loop_right = false }
                    else if d > self.next_state[(x + i) + (y+1)*self.width].density() {
                        dir = Some(true);
                        break;
                    }
                }
            }

            /* Bof
            let mut left: Option<usize> = None;
            
            for i in 1..=x {
                if d <= self.next_state[(x-i) + y*self.width].density() { break }
                else if d > self.next_state[(x-i) + (y+1)*self.width].density() {
                    dir = Some(false);
                    left = Some(i);
                    break;
                }
            }
            for i in 1..=(self.width-x-1) {
                if d <= self.next_state[(x+i) + y*self.width].density() { break }
                else if d > self.next_state[(x+i) + (y+1)*self.width].density() {
                    if let Some(left) = left {
                        dir = Some(i > left);
                    }
                    else {
                        dir = Some(true);
                    }
                    break;
                }
            }*/

            if let Some(dir) = dir {
                self.swap((x, y), (if dir { x+1 } else { x-1 }, y));
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Particle {
    Sand,
    Stone,
    Water,
    Oil,
    Void
}

impl Particle {
    fn color(&self) -> (u8, u8, u8) {
        match *self {
            Particle::Sand  => (255, 255,   0),
            Particle::Stone => (161, 161, 161),
            Particle::Water => (  0, 150, 255),
            Particle::Oil   => (189, 199, 86),
            Particle::Void  => (  0,   0,   0)
        }
    }

    fn density(&self) -> u8 {
        match *self {
            Particle::Sand  => 50,
            Particle::Stone => 255,
            Particle::Water => 10,
            Particle::Oil   => 9,
            Particle::Void  => 0
        }
    }

    pub fn from_rank(rank: usize) -> Particle {
        match rank {
            1 => Particle::Sand,
            2 => Particle::Stone,
            3 => Particle::Water,
            4 => Particle::Oil,
            _ => Particle::Void
        }
    }
}