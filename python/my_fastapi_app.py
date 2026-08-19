from fastapi import FastAPI
import os

# Пытаемся импортировать Rust-модуль, если он есть (опционально)
try:
    import jetapi_rs
    HAS_RUST = True
except ImportError:
    HAS_RUST = False
    print("Rust module not found, using pure Python compute")

app = FastAPI(title="JetAPI + FastAPI (embedded)")

@app.get("/")
async def root():
    return {"message": "Hello from FastAPI inside Rust!"}

@app.get("/compute")
async def compute(numbers: list[float]):
    if HAS_RUST:
        result = jetapi_rs.fast_compute(numbers)
    else:
        # fallback на Python
        result = [x * 2.0 for x in numbers]
    return {"result": result}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="127.0.0.1", port=8000)