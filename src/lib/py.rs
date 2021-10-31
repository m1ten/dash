use pyo3::prelude::*;
use std::io;
use std::fs::File;

#[pyfunction]
fn cmd(cmd: String, args: Vec<String>) -> PyResult<String> {
    let child = std::process::Command::new(cmd)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output()?;

    Ok(String::from_utf8(output.stdout).unwrap())
}

#[pyfunction]
fn get(_py: Python, url: String, file: String) -> u64{
    let mut resp = reqwest::blocking::get(url).expect("Failed to get");
    let mut out = File::create(file).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy")
}

#[pymodule]
fn dash(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cmd, m)?)?;
    m.add_function(wrap_pyfunction!(get, m)?)?;

    Ok(())
}