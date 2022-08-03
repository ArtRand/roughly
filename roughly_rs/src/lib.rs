use pyo3::prelude::*;
use pyo3::{PyResult, Python, wrap_pyfunction};
use pyo3::types::{PyDict, PyModule};
use roughlylib::aligner;


#[pyfunction]
fn align_dna(_py: Python, a: &str, b: &str, sub_matrix: &PyDict) -> PyResult<()>{
    let mismatch_score = sub_matrix.get_item("match")
        .map(|e| e.extract::<f64>())
        .unwrap_or(Ok(-2.0))?;



    Ok(())
}

#[pyfunction]
fn foobar(_py: Python, s: &str) -> PyResult<String> {
    Ok(format!("hello {}", s))
}

#[pymodule]
fn roughly_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(foobar, m)?)?;

    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
