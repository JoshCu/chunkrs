use pyo3::prelude::*;

mod cli;
mod downloader;
mod utils;

#[pymodule]
fn dlpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(downloader::download, m)?)?;
    m.add_function(wrap_pyfunction!(cli::cli_main, m)?)?;
    Ok(())
}
