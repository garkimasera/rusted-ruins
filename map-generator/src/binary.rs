use geom::*;

pub fn create_binary_fractal(size: Vec2d, weight: f32) -> Array2d<bool> {
    let mut map = Array2d::new(size.0 as u32, size.1 as u32, false);
    let height = crate::fractal::create_fractal(size);

    for p in height.iter_idx() {
        if height[p] < weight {
            map[p] = true;
        }
    }

    map
}
