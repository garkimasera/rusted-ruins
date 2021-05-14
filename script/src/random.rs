use rustpython_vm::pymodule;

pub(crate) use _random::make_module;

#[pymodule(name = "random")]
mod _random {
    use rusted_ruins_rng as rng;

    #[pyfunction]
    fn randint(a: u64, b: u64) -> u64 {
        rng::gen_range(a..=b)
    }
}
