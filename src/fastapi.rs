//! Модуль для запуска FastAPI-сервера из Rust через PyO3.
//! Доступен только при включенном feature "fastapi".

#[cfg(feature = "fastapi")]
use pyo3::prelude::*;
#[cfg(feature = "fastapi")]
use pyo3::types::PyDict;
#[cfg(feature = "fastapi")]
use std::env;
#[cfg(feature = "fastapi")]
use std::path::PathBuf;

#[cfg(feature = "fastapi")]
/// Запускает FastAPI-сервер, используя Python-скрипт из папки `python/`.
/// Параметры: `host` и `port` (по умолчанию 127.0.0.1:8000).
pub fn run_fastapi(host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // Инициализируем интерпретатор Python
    pyo3::prepare_freethreaded_python();

    // Получаем путь к текущему исполняемому файлу или к корню проекта
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let python_dir = PathBuf::from(manifest_dir).join("python");

    // Добавляем папку python в sys.path
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let path: &pyo3::types::PyList = sys.getattr("path")?.downcast()?;
        path.insert(0, python_dir.to_str().unwrap())?;
        Ok(())
    })?;

    // Запускаем uvicorn из Python
    Python::with_gil(|py| {
        let uvicorn = py.import("uvicorn")?;
        let app_path = format!("my_fastapi_app:app");
        let kwargs = PyDict::new(py);
        kwargs.set_item("host", host)?;
        kwargs.set_item("port", port)?;
        kwargs.set_item("log_level", "info")?;
        // Запускаем uvicorn.run(app, **kwargs)
        uvicorn.call_method("run", (app_path,), Some(kwargs))?;
        Ok::<_, pyo3::PyErr>(())
    })?;

    Ok(())
}

#[cfg(not(feature = "fastapi"))]
/// Заглушка, если feature не включён.
pub fn run_fastapi(_host: &str, _port: u16) -> Result<(), Box<dyn std::error::Error>> {
    panic!("Feature 'fastapi' is not enabled. Add 'jetapi = {{ features = [\"fastapi\"] }}' to your Cargo.toml.")
}