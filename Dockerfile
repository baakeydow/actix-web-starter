FROM debian:bookworm as builder

ENV RUNTIME_FOLDER=/root/workspace \
    RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=%%RUST-VERSION%%

WORKDIR ${RUNTIME_FOLDER}

COPY . ${RUNTIME_FOLDER}

RUN apt-get update -y && apt-get install build-essential cmake curl -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . $CARGO_HOME/env && \
    cargo install --path .

# final docker image

FROM debian:bookworm-slim

ENV RUNTIME_FOLDER=/root/workspace

WORKDIR ${RUNTIME_FOLDER}

COPY --from=builder /usr/local/cargo/bin/actix-web-starter /usr/local/bin

EXPOSE 1342

VOLUME [ "/root/workspace/runtime" ]

ENTRYPOINT [ "/bin/bash", "-l", "-c" ]

CMD ["actix-web-starter"]

