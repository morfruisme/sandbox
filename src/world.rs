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
                let c = self.state[x + y*self.width].get_color();
                frame[4*(x + y*w) + 0] = c.0;
                frame[4*(x + y*w) + 1] = c.1;
                frame[4*(x + y*w) + 2] = c.2;
            }
            let p = Particle::from_rank(y);
            let c = p.get_color();
            frame[4*(y+1)*w - 4] = c.0;
            frame[4*(y+1)*w - 3] = c.1;
            frame[4*(y+1)*w - 2] = c.2;

            frame[4*(y+1)*w - 8] = if p == selected_particle { 255 } else { 0 };
        }
    }

    pub fn spawn(&mut self, x: usize, y: usize, t: Particle) {
        self.state[x + y*self.width] = t;
    }

    pub fn update(&mut self) {
        for y in (0..self.height).rev() {
            for x in (0..self.width).rev() {
                match self.state[x + y*self.width] {
                    Particle::Sand/*(v)*/ => self.update_sand(x, y), //v),
                    Particle::Stone/*(v)*/ => self.update_stone(x, y), //v),
                    Particle::Water => self.upate_water(x, y),
                    Particle::Void => ()
                }
            }
        }
        self.update_buffers();
    }

    fn update_buffers(&mut self) {
        for (p, new_p) in self.state.iter_mut().zip(self.next_state.iter()) {
            *p = *new_p;
        }
    }

    fn _swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let t = self.next_state[x1 + y1*self.width];
        self.next_state[x1 + y1*self.width] = self.next_state[x2 + y2*self.width];
        self.next_state[x2 + y2*self.width] = t;
    }

    // TODO: Fix sand/water interactions
    fn update_sand(&mut self, x: usize, y: usize) {//, v: u8) {
        if y == self.height-1 { return }

        // TODO: fix
        /*let d = self.next_state[x + y*self.width].get_density();
        if d > self.next_state[x + (y+1)*self.width].get_density() {
            self.swap(x, y, x, y+1)
        }
        else {
            if d > self.next_state[(x-1) + (y+1)*self.width].get_density() {
                self.swap(x, y, x-1, y+1)
            }
            else if d > self.next_state[(x+1) + (y+1)*self.width].get_density() {
                self.swap(x, y, x+1, y+1)
            }
            else {
                self.next_state[x + y*self.width] = Particle::Sand;
            }
        }*/

        match self.next_state[x + (y+1)*self.width] {
            Particle::Void => {
                self.next_state[x + y*self.width] = Particle::Void;
                self.next_state[x + (y+1)*self.width] = Particle::Sand;//(v);
            }
            Particle::Water => {
                self.next_state[x + y*self.width] = Particle::Water;
                self.next_state[x + (y+1)*self.width] = Particle::Sand;//(v);
            }
            _ => {
                if x != 0 && self.next_state[(x-1) + (y+1)*self.width] == Particle::Void && self.next_state[(x-1) + y*self.width] == Particle::Void {
                    self.next_state[(x-1) + (y+1)*self.width] = Particle::Sand;//(v);
                    self.next_state[x + y*self.width] = Particle::Void;
                }
                else if x != self.width-1 && self.next_state[(x+1) + (y+1)*self.width] == Particle::Void && self.next_state[(x+1) + y*self.width] == Particle::Void {
                    self.next_state[(x+1) + (y+1)*self.width] = Particle::Sand;//(v);
                    self.next_state[x + y*self.width] = Particle::Void;
                }
                else {
                    self.next_state[x + y*self.width] = Particle::Sand;//(v);
                }
            }
        }
    }

    fn update_stone(&mut self, x: usize, y: usize) {//, v: u8) {
        self.next_state[x + y*self.width] = Particle::Stone;//(v);
    }

    fn upate_water(&mut self, x: usize, y: usize) {
        if y == self.height-1 { return }

        match self.next_state[x + (y+1)*self.width] {
            Particle::Void => {
                self.next_state[x + (y+1)*self.width] = Particle::Water;
                self.next_state[x + y*self.width] = Particle::Void;
            },
            _ => {
                let mut dir: Option<bool> = None;
                for i in 1..=x {
                    if self.next_state[(x-i) + y*self.width] != Particle::Void { break }
                    else if self.next_state[(x-i) + (y+1)*self.width] == Particle::Void {
                        dir = Some(false);
                        break;
                    }
                }
                for i in 1..=(self.width-x-1) {
                    if self.next_state[(x+i) + y*self.width] != Particle::Void { break }
                    else if self.next_state[(x+i) + (y+1)*self.width] == Particle::Void {
                        dir = Some(true);
                        break;
                    }
                }

                match dir {
                    Some(dir) => {
                        self.next_state[(if dir { x+1 } else { x-1 }) + y*self.width] = Particle::Water;
                        self.next_state[x + y*self.width] = Particle::Void;
                    },
                    None => self.next_state[x + y*self.width] = Particle::Water
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Particle {
    Sand,//(u8),
    Stone,//(u8),
    Water,
    Void
}

impl Particle {
    fn get_color(&self) -> (u8, u8, u8) {
        match *self {
            Particle::Sand  => (255, 255,   0),
            Particle::Stone => (161, 161, 161),
            Particle::Water => (  0, 150, 255),
            Particle::Void  => (  0,   0,   0)
        }
    }

    fn _get_density(&self) -> u8 {
        match *self {
            Particle::Sand  => 5,
            Particle::Stone => 10,
            Particle::Water => 1,
            Particle::Void  => 0
        }
    }

    fn from_rank(rank: usize) -> Particle {
        match rank {
            1 => Particle::Sand,
            2 => Particle::Stone,
            3 => Particle::Water,
            _ => Particle::Void
        }
    }
}