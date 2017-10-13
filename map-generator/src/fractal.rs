
use array2d::*;
use rand::{Rng, thread_rng};
use super::{GeneratedMap, TileKind};

pub fn write_to_map(gm: &mut GeneratedMap) {
    let fractal = create_fractal(gm.size);

    let threshold = calc_threshold(&fractal, 0.6);
    
    for p in gm.tile.iter_idx() {
        if fractal[p] > threshold {
            gm.tile[p] = TileKind::Wall;
        }else{
            gm.tile[p] = TileKind::Floor;
        }
    }

    // Determine start and end
    let start = pick_passable_tile(&gm);
    let reach_map = create_reach_map(&gm, start);

    loop {
        let end = pick_passable_tile(&gm);
        if start != end && reach_map[end] {
            gm.entrance = start;
            gm.exit = Some(end);
            break;
        }
    }

    // Write walls for unreachable tiles from the start
    for p in gm.tile.iter_idx() {
        if !reach_map[p] && gm.tile[p] != TileKind::Wall {
            gm.tile[p] = TileKind::Wall;
        }
    }
}

pub fn create_fractal(size: Vec2d) -> Array2d<f32> {
    let mut map = Array2d::new(size.0 as u32, size.1 as u32, 0.0);

    // Biasing for edges
    let edge_bias = [3.0, 1.5, 1.0, 0.5];
    for (i, b) in edge_bias.iter().enumerate() {
        let i = i as i32;
        println!("{}", i);
        write_rect(&mut map, *b, Vec2d::new(i, i), Vec2d::new(size.0 - i - 1, size.1 - i - 1));
    }

    write_block(&mut map, 8, 1.0);
    write_block(&mut map, 7, 1.0);
    write_block(&mut map, 6, 1.0);
    write_block(&mut map, 5, 1.0);
    write_block(&mut map, 4, 1.0);
    write_block(&mut map, 2, 1.0);
    write_block(&mut map, 1, 1.0);

    // Normalization
    let mut max = 0.0;
    for p in map.iter_idx() {
        if max < map[p] {
            max = map[p];
        }
    }
    let mut min = max;
    for p in map.iter_idx() {
        if min > map[p] {
            min = map[p];
        }
    }
    for p in map.iter_idx() {
        map[p] = (map[p] - min) / (max - min);
    }
    
    map
}

fn write_block(map: &mut Array2d<f32>, block_size: u32, weight: f32) {
    let size = map.size();
    let nx_block = size.0 / block_size + 1;
    let ny_block = size.1 / block_size + 1;
    let mut rand_map = Array2d::new(nx_block, ny_block, 0.0f32);

    let mut rng = thread_rng();
    for p in rand_map.iter_idx() {
        rand_map[p] = rng.gen_range(0.0, weight);
    }

    for p in map.iter_idx() {
        map[p] += rand_map[(p.0 / block_size as i32, p.1 / block_size as i32)];
    }
}

fn write_rect(map: &mut Array2d<f32>, value: f32, top_left: Vec2d, bottom_right: Vec2d) {
    let top_right = Vec2d::new(bottom_right.0, top_left.1);
    let bottom_left = Vec2d::new(top_left.0, bottom_right.1);

    for p in LineIter::new(top_left, top_right) {
        map[p] = value;
    }
    for p in LineIter::new(top_right, bottom_right) {
        map[p] = value;
    }
    for p in LineIter::new(bottom_right, bottom_left) {
        map[p] = value;
    }
    for p in LineIter::new(bottom_left, top_left) {
        map[p] = value;
    }
}

fn calc_threshold(fractal: &Array2d<f32>, floor_ratio: f32) -> f32 {
    let n_tile = fractal.size().0 * fractal.size().1;

    let mut v: Vec<f32> = fractal.iter().map(|a| *a).collect();
    v.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    v[(n_tile as f32 * floor_ratio) as usize]
}

/// Calculate tiles are reacheable from given tile
fn create_reach_map(map: &GeneratedMap, start: Vec2d) -> Array2d<bool> {
    let mut reachable = Array2d::new(map.size.0 as u32, map.size.1 as u32, false);

    if map.tile[start].is_passable() {
        reachable[start] = true;
    }else{
        return reachable;
    }

    loop {
        let mut new_reachable_tile = false;

        for p in map.tile.iter_idx() {
            if reachable[p] {
                let mut try_next_tile = |next_tile: Vec2d| {
                    if map.tile.in_range(next_tile) && map.tile[next_tile].is_passable()
                        && !reachable[next_tile] {
                        reachable[next_tile] = true; new_reachable_tile = true;
                    }
                };
                
                try_next_tile(p + (-1,  0));
                try_next_tile(p + ( 1,  0));
                try_next_tile(p + ( 0, -1));
                try_next_tile(p + ( 0,  1));
            }
        }

        if !new_reachable_tile { break; }
    }

    reachable
}

/// Pick one passable tile at random
fn pick_passable_tile(map: &GeneratedMap) -> Vec2d {
    let mut rng = thread_rng();
    
    loop {
        let p = Vec2d(rng.gen_range(0, map.size.0), rng.gen_range(0, map.size.1));

        if map.tile[p].is_passable() { return p; }
    }
}

