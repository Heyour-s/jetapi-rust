use pyo3::prelude::*;

/// Быстрая Rust-функция для тяжёлых вычислений
#[pyfunction]
fn fast_compute(data: Vec<f64>) -> PyResult<Vec<f64>> {
    // Здесь можно использовать Axum, Tokio, делать сложные расчёты
    let result: Vec<f64> = data.iter().map(|x| x * 2.0).collect();
    Ok(result)
}

/// Модуль, который будет импортироваться в Python
#[pymodule]
fn jetapi_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fast_compute, m)?)?;
    Ok(())
}