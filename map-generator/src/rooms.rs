use super::{GeneratedMap, TileKind};
use array2d::*;
use rand::seq::IteratorRandom;
use rng::{gen_range, GameRng};

const MAX_TRY: usize = 256;

struct Room {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    /// One room can have doors for each direction (N,E,S,W)
    has_door: [bool; 4],
}

impl Room {
    fn intersect_or_touch(&self, other: &Room) -> bool {
        let left0 = self.x - 1;
        let left1 = other.x;
        let right0 = self.x + self.w as i32 + 1;
        let right1 = other.x + other.w as i32;
        let top0 = self.y - 1;
        let top1 = other.y;
        let bottom0 = self.y + self.h as i32 + 1;
        let bottom1 = other.y + other.h as i32;

        !(left1 > right0 || right1 < left0 || top1 > bottom0 || bottom1 < top0)
    }
}

pub struct Rooms {
    max_room_size: u32,
    min_room_size: u32,
    n_room: u32,
}

impl Rooms {
    pub fn new(max_room_size: u32, min_room_size: u32, n_room: u32) -> Rooms {
        Rooms {
            max_room_size,
            min_room_size,
            n_room,
        }
    }

    pub fn write_to_map(&self, gm: &mut GeneratedMap) {
        let size = gm.size;

        for p in gm.tile.iter_idx() {
            gm.tile[p] = TileKind::Wall;
        }

        let mut rooms = vec![self.create_start_room(size)];
        let mut doors = Vec::new();

        for _ in 0..self.n_room {
            if let Err(_) = self.locate_new_room(&mut rooms, &mut doors, size) {
                break;
            }
        }

        assert!(rooms.len() >= 2);

        let mut rooms_with_stairs: [usize; 2] = [0, 0];
        (0..rooms.len()).choose_multiple_fill(&mut GameRng, &mut rooms_with_stairs);

        for (i, room) in rooms.iter().enumerate() {
            let rectiter = RectIter::new(
                (room.x, room.y),
                (room.x + room.w as i32, room.y + room.h as i32),
            );
            for p in rectiter {
                gm.tile[p] = TileKind::Floor;
            }
            if i == rooms_with_stairs[0] || i == rooms_with_stairs[1] {
                let dx = gen_range(1, room.w - 1) as i32;
                let dy = gen_range(1, room.h - 1) as i32;
                let stair_tile = Vec2d(room.x + dx, room.y + dy);
                if i == rooms_with_stairs[0] {
                    gm.entrance = stair_tile;
                } else {
                    gm.exit = Some(stair_tile);
                }
            }
        }

        for p in doors {
            gm.tile[p] = TileKind::Door;
        }
    }

    fn locate_new_room(
        &self,
        rooms: &mut Vec<Room>,
        doors: &mut Vec<Vec2d>,
        size: Vec2d,
    ) -> Result<(), ()> {
        // Search empty wall that does not have a door
        let n_empty_wall = rooms
            .iter()
            .flat_map(|room| &room.has_door)
            .filter(|d| !**d)
            .count();

        'try_loop: for _ in 0..MAX_TRY {
            // Choose wall to dig

            let i_roomwall = gen_range(0, n_empty_wall);
            let mut a = i_roomwall;
            let (i_room, i_wall) = 'room_loop: loop {
                for (i_room, room) in rooms.iter().enumerate() {
                    for (i_wall, wall) in room.has_door.iter().enumerate() {
                        if !wall {
                            if a == 0 {
                                break 'room_loop (i_room, i_wall);
                            }
                            a -= 1;
                        }
                    }
                }
                unreachable!();
            };

            let dir = match i_wall {
                0 => Direction::N,
                1 => Direction::E,
                2 => Direction::S,
                3 => Direction::W,
                _ => unreachable!(),
            };

            let parent = &rooms[i_room];

            // Make new door
            let new_door_pos = Vec2d::from(match dir {
                Direction::N => (parent.x + gen_range(0, parent.w as i32), parent.y - 1),
                Direction::E => (
                    parent.x + parent.w as i32,
                    parent.y + gen_range(0, parent.h as i32),
                ),
                Direction::S => (
                    parent.x + gen_range(0, parent.w as i32),
                    parent.y + parent.h as i32,
                ),
                Direction::W => (parent.x - 1, parent.y + gen_range(0, parent.h as i32)),
                _ => unreachable!(),
            });

            // Make new room
            let new_room_w = gen_range(self.min_room_size, self.max_room_size + 1);
            let new_room_h = gen_range(self.min_room_size, self.max_room_size + 1);
            let new_room_top_left_door = match dir {
                Direction::N => (
                    new_door_pos.0 - gen_range(0, new_room_w as i32),
                    new_door_pos.1 - new_room_h as i32 - 1,
                    [false, false, true, false],
                ),
                Direction::E => (
                    new_door_pos.0 + 1,
                    new_door_pos.1 - gen_range(0, new_room_h as i32),
                    [false, false, false, true],
                ),
                Direction::S => (
                    new_door_pos.0 - gen_range(0, new_room_w as i32),
                    new_door_pos.1 + 1,
                    [true, false, false, false],
                ),
                Direction::W => (
                    new_door_pos.0 - new_room_w as i32 - 1,
                    new_door_pos.1 - gen_range(0, new_room_h as i32),
                    [false, true, false, false],
                ),
                _ => unreachable!(),
            };
            let new_room = Room {
                x: new_room_top_left_door.0,
                y: new_room_top_left_door.1,
                w: new_room_w,
                h: new_room_h,
                has_door: new_room_top_left_door.2,
            };

            // Check new room is suitable
            for room in rooms.iter() {
                if new_room.intersect_or_touch(room) {
                    continue 'try_loop;
                }
            }
            if new_room.x <= 1
                || new_room.y <= 1
                || new_room.x + new_room.w as i32 >= size.0 - 1
                || new_room.y + new_room.h as i32 >= size.1 - 1
            {
                continue 'try_loop;
            }

            // Push new room
            rooms[i_room].has_door[i_wall] = true;
            rooms.push(new_room);
            doors.push(new_door_pos);

            return Ok(());
        }

        Err(())
    }

    fn create_start_room(&self, size: Vec2d) -> Room {
        let center_tile = (size.0 / 2, size.1 / 2);

        let room_size = self.gen_room_size();

        Room {
            x: center_tile.0 - gen_range(0, room_size.0 / 2),
            y: center_tile.1 - gen_range(0, room_size.1 / 2),
            w: room_size.0 as u32,
            h: room_size.1 as u32,
            has_door: [false; 4],
        }
    }

    fn gen_room_size(&self) -> Vec2d {
        Vec2d(
            gen_range(self.min_room_size, self.max_room_size + 1) as i32,
            gen_range(self.min_room_size, self.max_room_size + 1) as i32,
        )
    }
}
