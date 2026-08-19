//! Обёртки над middleware из poem.

pub use poem::middleware::{
    AddData,
    Cors,
    NormalizePath,
    Compression,
    Tracing,
    SizeLimit,
    CatchPanic,
    ForceHttps,
    SetHeader,
    SensitiveHeader,
    PropagateHeader,
};

// Timeout не экспортируем – пользователь может использовать его напрямую:
// use poem::middleware::timeout::Timeout;