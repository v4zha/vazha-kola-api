FROM rust:1.49 as builder
COPY ./ ./
RUN cargo build --release
CMD [ "./target/release/vazha-kola-api" ]
