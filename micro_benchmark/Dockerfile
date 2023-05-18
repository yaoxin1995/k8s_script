# 1. This tells docker to use the Rust official image
FROM rust:1.65

# 2. Copy the files in your machine to the Docker image
WORKDIR /usr/src/app
COPY ./main /usr/src/app

# Build your program for release
# RUN cargo build

# Run the binary
CMD ["/usr/src/app/main"]
