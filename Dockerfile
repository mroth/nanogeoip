ARG TOOLCHAIN=x86_64-unknown-linux-musl

FROM rust:1.31-slim as builder
ARG TOOLCHAIN
RUN rustup target add $TOOLCHAIN

WORKDIR /home/rust

# hacky way to force just dep compilation for layer caching,
# in an ideal world this really needs to be enabled in cargo,
# with some sort of command to only compile deps... sigh.
#
# we need at least a main so cargo will build our deps
RUN mkdir -p src
RUN echo "fn main() {}" > src/main.rs
#
# now we need the same for the criterion benchmarks, otherwise
# cargo will throw a "unable to parse manifest" error with this
# detail owing to how criterion is activated in Cargo.toml
#
#  `can't find `http_benchmarks` bench, specify bench.path`
RUN mkdir -p benches
RUN echo "fn main() {}" > benches/http_benchmarks.rs
# finally just copy of the actual cargo files and do a build, which
# will compile only the deps.
COPY Cargo.toml Cargo.lock ./
RUN cargo build --target $TOOLCHAIN --release

# compile the actual program
COPY src/*.rs src/
RUN cargo build --target $TOOLCHAIN --release

# size optimization
RUN strip target/${TOOLCHAIN}/release/nanogeoip


FROM scratch
ARG TOOLCHAIN
WORKDIR /home/rust/
COPY --from=builder /home/rust/target/${TOOLCHAIN}/release/nanogeoip .
ENTRYPOINT ["./nanogeoip"]
