FROM rust:1.74

WORKDIR /code

RUN rustup component add rustfmt clippy

ENTRYPOINT ["task", "run"]