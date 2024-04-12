FROM rust:latest as builder

WORKDIR /app

# create a new empty project
RUN cargo init

COPY ./.cargo .cargo
# COPY ./vendor vendor # Commented out temporarely
COPY Cargo.toml Cargo.lock ./
# build dependencies, when my source code changes, this build can be cached, we don't need to compile dependency again.
RUN cargo build
# remove the dummy build.
RUN cargo clean -p $project_name_specified_in_cargo

RUN cargo install --path .

# second stage.
FROM gcr.io/distroless/cc-debian11
COPY --from=builder /usr/local/cargo/bin/* /usr/local/bin
