FROM rustlang/rust:nightly-slim
ADD src /root/code/src
ADD Cargo.toml /root/code/
WORKDIR /root/code
RUN cargo build --release
ENTRYPOINT ["/root/code/target/release/benchmark-containers"]
CMD []