FROM rust:latest as Builder

ARG WORD_DIR=/app
WORKDIR ${WORD_DIR}
COPY src ./src
COPY Cargo.toml Cargo.toml

RUN cargo build --release


FROM debian:bullseye-slim

ARG WORD_DIR=/app
WORKDIR ${WORD_DIR}

ARG APP=msg-s-vangari

COPY Rocket.toml Rocket.toml
COPY --from=Builder ${WORD_DIR}/target/release/msg-s-vangari ${WORD_DIR}/application
CMD ["./application"]
