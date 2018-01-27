
use array2d::*;
use rng::*;
use super::{GeneratedMap, TileKind};

pub struct Lattice {
    nx: u32,
    ny: u32,
    ew_open: Array2d<bool>,
    ns_open: Array2d<bool>,
    start: Vec2d,
    end: Vec2d,
}

impl Lattice {
    fn new(nx: u32, ny: u32) -> Lattice {
        Lattice {
            nx,
            ny,
            ew_open: Array2d::new(nx - 1, ny, false),
            ns_open: Array2d::new(nx, ny - 1, false),
            start: Vec2d::new(0, 0),
            end: Vec2d::new(0, 0),
        }
    }

    pub fn write_to_map(&self, gm: &mut GeneratedMap, door_weight: f64) {
        let ew_wall_len = (gm.size.0 - self.nx as i32 + 1) / self.nx as i32;
        let ns_wall_len = (gm.size.1 - self.ny as i32 + 1) / self.ny as i32;
        
        // Set entrance/exit
        gm.entrance = Vec2d::new(
            self.start.0 * (ew_wall_len + 1) + ew_wall_len / 2,
            self.start.1 * (ns_wall_len + 1) + ns_wall_len / 2);
        gm.exit = Some(Vec2d::new(
            self.end.0 * (ew_wall_len + 1) + ew_wall_len / 2,
            self.end.1 * (ns_wall_len + 1) + ns_wall_len / 2));

        // Write horizontal walls
        for b in 0..(self.ny as i32 - 1) {
            let j = (ns_wall_len + 1) * (b + 1) - 1;

            for a in 0..(self.nx as i32) {
                if self.ns_open[(a, b)] {
                    if door_weight > gen_range(0.0, 1.0) {
                        let middle = ew_wall_len / 2;
                        for c in 0..ew_wall_len {
                            if c == middle {
                                gm.tile[((ew_wall_len + 1) * a + c, j)] = TileKind::Door;
                            }else{
                                gm.tile[((ew_wall_len + 1) * a + c, j)] = TileKind::Wall;
                            }
                        }
                    }
                }else{
                    for c in 0..ew_wall_len {
                        gm.tile[((ew_wall_len + 1) * a + c, j)] = TileKind::Wall;
                    }
                }
            }
        }

        // Write vertical walls
        for a in 0..(self.nx as i32 - 1) {
            let i = (ew_wall_len + 1) * (a + 1) - 1;

            for b in 0..(self.ny as i32) {
                if self.ew_open[(a, b)] {
                    // Determine door or fully opened
                    if door_weight > gen_range(0.0, 1.0) {
                        let middle = ns_wall_len / 2;
                        for c in 0..ns_wall_len {
                            if c == middle {
                                gm.tile[(i, (ns_wall_len + 1) * b + c)] = TileKind::Door;
                            }else{
                                gm.tile[(i, (ns_wall_len + 1) * b + c)] = TileKind::Wall;
                            }
                        }
                    }
                }else{
                    for c in 0..ns_wall_len {
                        gm.tile[(i, (ns_wall_len + 1) * b + c)] = TileKind::Wall;
                    }
                }
            }
        }

        // Write cross point
        for a in 0..(self.nx as i32 - 1) {
            for b in 0..(self.ny as i32 - 1) {
                let i = (ew_wall_len + 1) * a + ew_wall_len;
                let j = (ns_wall_len + 1) * b + ns_wall_len;
                gm.tile[(i, j)] = TileKind::Wall;
            }
        }
    }
}

#[derive(Debug)]
enum Dir {
    W,
    E,
    N,
    S,
}

pub fn create_lattice(nx: u32, ny: u32, min_step: u32, max_step: u32) -> Lattice {
    let mut lattice = Lattice::new(nx, ny);
    let mut is_reach = Array2d::new(nx, ny, false);

    let start_room = Vec2d::new(gen_range(0, nx) as i32, gen_range(0, ny) as i32);
    lattice.start = start_room;

    let max_step = gen_range(min_step, max_step);
    
    // Determine start and goal, and the route
    random_walk(start_room,
                &mut lattice,
                &mut is_reach,
                0,
                max_step);

    // Make all room reachable
    'room_scan: loop {
        let right_bottom = Vec2d::new(nx as i32, ny as i32);
        for room in right_bottom.iter_from_zero() {
            if is_reach[room] {
                continue;
            }

            // If next room is reachable, pushes it.
            let mut next_rooms = Vec::new();
            let west_room = room + (-1, 0);
            if room.0 > 0 && is_reach[west_room] {
                next_rooms.push(Dir::W);
            }
            let east_room = room + (1, 0);
            if room.0 < lattice.nx as i32 - 1 && is_reach[east_room] {
                next_rooms.push(Dir::E);
            }
            let north_room = room + (0, -1);
            if room.1 > 0 && is_reach[north_room] {
                next_rooms.push(Dir::N);
            }
            let south_room = room + (0, 1);
            if room.1 < lattice.ny as i32 - 1 && is_reach[south_room] {
                next_rooms.push(Dir::S);
            }
            // Random select which wall will be opened.
            if let Some(next_room) = get_rng().choose(&next_rooms) {
                match *next_room {
                    Dir::W => {
                        lattice.ew_open[(room.0 - 1, room.1)] = true;
                    }
                    Dir::E => {
                        lattice.ew_open[(room.0, room.1)] = true;
                    }
                    Dir::N => {
                        lattice.ns_open[(room.0, room.1 - 1)] = true;
                    }
                    Dir::S => {
                        lattice.ns_open[(room.0, room.1)] = true;
                    }
                }
                is_reach[room] = true;
            }
        }

        for room in right_bottom.iter_from_zero() {
            if !is_reach[room] {
                continue 'room_scan;
            }
        }
        break;
    }


    lattice
}

fn random_walk(room: Vec2d,
               lattice: &mut Lattice,
               is_reach: &mut Array2d<bool>,
               count: u32,
               n_step: u32) {

    let mut next_rooms = Vec::new();
    is_reach[room] = true;

    let west_room = room + (-1, 0);
    if room.0 > 0 && !is_reach[west_room] {
        next_rooms.push(Dir::W);
    }
    let east_room = room + (1, 0);
    if room.0 < lattice.nx as i32 - 1 && !is_reach[east_room] {
        next_rooms.push(Dir::E);
    }
    let north_room = room + (0, -1);
    if room.1 > 0 && !is_reach[north_room] {
        next_rooms.push(Dir::N);
    }
    let south_room = room + (0, 1);
    if room.1 < lattice.ny as i32 - 1 && !is_reach[south_room] {
        next_rooms.push(Dir::S);
    }

    if count >= n_step {
        lattice.end = room;
        return;
    }

    if let Some(next_room) = get_rng().choose(&next_rooms) {
        match *next_room {
            Dir::W => {
                lattice.ew_open[(room.0 - 1, room.1)] = true;
                random_walk(west_room, lattice, is_reach, count + 1, n_step);
            }
            Dir::E => {
                lattice.ew_open[(room.0, room.1)] = true;
                random_walk(east_room, lattice, is_reach, count + 1, n_step);
            }
            Dir::N => {
                lattice.ns_open[(room.0, room.1 - 1)] = true;
                random_walk(north_room, lattice, is_reach, count + 1, n_step);
            }
            Dir::S => {
                lattice.ns_open[(room.0, room.1)] = true;
                random_walk(south_room, lattice, is_reach, count + 1, n_step);
            }
        }
    } else {
        lattice.end = room;
        return;
    }
}

impl ::std::fmt::Display for Lattice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        for j in 0..self.ny {
            for i in 0..self.nx {
                if self.start == Vec2d::new(i as i32, j as i32) {
                    write!(f, "S")?;
                } else if self.end == Vec2d::new(i as i32, j as i32) {
                    write!(f, "E")?;
                } else {
                    write!(f, ".")?;
                }

                if i != self.nx - 1 {
                    if self.ew_open[(i, j)] {
                        write!(f, "D")?;
                    } else {
                        write!(f, "#")?;
                    }
                }
            }
            write!(f, "\n")?;
            if j != self.ny - 1 {
                for i in 0..self.nx {
                    if self.ns_open[(i, j)] {
                        write!(f, "D")?;
                    } else {
                        write!(f, "#")?;
                    }
                    if i != self.nx - 1 {
                        write!(f, "#")?;
                    }
                }
                write!(f, "\n")?;
            }
        }


        Ok(())
    }
}
