use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = openspeleo_core::stub_info()?;
    stub.generate()?;
    Ok(())
}
