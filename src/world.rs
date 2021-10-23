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

    pub fn draw(&self, frame: &mut [u8]) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.state[x + y*self.width] {
                    Particle::Sand/*(v)*/ => {
                        frame[4*(x + y*self.width) + 0] = 200;
                        frame[4*(x + y*self.width) + 1] = 190;// + v;
                        frame[4*(x + y*self.width) + 2] = 0;
                    },
                    Particle::Stone/*(v)*/ => {
                        frame[4*(x + y*self.width) + 0] = 84;// + v;
                        frame[4*(x + y*self.width) + 1] = 90;// + v;
                        frame[4*(x + y*self.width) + 2] = 96;// + v;
                    },
                    Particle::Water => {
                        frame[4*(x + y*self.width) + 0] = 0;
                        frame[4*(x + y*self.width) + 1] = 150;
                        frame[4*(x + y*self.width) + 2] = 255;
                    }
                    Particle::Void => {
                        frame[4*(x + y*self.width) + 0] = 0;
                        frame[4*(x + y*self.width) + 1] = 0;
                        frame[4*(x + y*self.width) + 2] = 0;
                    }
                }
            }
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

    fn update_sand(&mut self, x: usize, y: usize) {//, v: u8) {
        if y == self.height-1 { return }

        match self.next_state[x + (y+1)*self.width] {
            Particle::Void => {
                self.next_state[x + y*self.width] = Particle::Void;
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