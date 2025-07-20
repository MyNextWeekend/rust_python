FROM ghcr.io/pyo3/maturin:buildcache

WORKDIR /app

COPY ./python .
COPY ./src .
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./pyproject.toml .

RUN maturin build --release --strip 



