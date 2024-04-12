FROM rust:1.77 as build

# create a new empty shell project
RUN USER=root cargo new --bin streaming-client-rust
WORKDIR /holodeck

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/streaming-client-desktop*
RUN cargo build --release

# our final base
FROM rust:1.77

# copy the build artifact from the build stage
COPY --from=build /holodeck/target/release/streaming-client-desktop .

# set the startup command to run your binary
CMD ["./streaming-client-desktop"]
