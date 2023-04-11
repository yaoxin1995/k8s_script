# 1. This tells docker to use the Rust official image
FROM rust:1.66

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# Build your program for release
RUN cargo build

# Run the binary
CMD ["./target/debug/test_get_report_syscall"]
