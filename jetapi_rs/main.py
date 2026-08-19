from fastapi import FastAPI
import jetapi_rs

app = FastAPI(title="JetAPI + FastAPI гибрид")

@app.get("/compute")
async def compute(numbers: list[float]):
    # Вызываем быструю Rust-функцию
    result = jetapi_rs.fast_compute(numbers)
    return {"result": result}

@app.get("/")
async def root():
    return {"message": "Hello from FastAPI with Rust power!"}