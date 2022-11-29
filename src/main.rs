use aoc_lib::TracingAlloc;
use color_eyre::Result;

mod days;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc;

fn main() -> Result<()> {
    color_eyre::install()?;
    aoc_lib::run(&ALLOC, 2022, days::DAYS)?;

    Ok(())
}
