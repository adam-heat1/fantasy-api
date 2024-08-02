# Rust as the base image
FROM rust:1.75 as build

# 1. Create a new empty shell project
RUN USER=root cargo new --bin fantasy-api
WORKDIR /fantasy-api

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/fantasy_api*
RUN cargo build --release

FROM rust:1.74

COPY --from=build /fantasy-api/target/release/fantasy-api .

CMD ["./fantasy-api"]