# Use an official Rust image to build the app
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo files and application code to the container
COPY . .

# Build the application in release mode
RUN cargo build --release

# Use Ubuntu 22.04 as a base image for running the app (with glibc 2.32 or higher)
FROM ubuntu:22.04

# Install any necessary runtime dependencies (e.g., OpenSSL if required)
RUN apt-get update && apt-get install -y libssl-dev

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/personal-website /usr/local/bin/personal-website
COPY ./posts /posts
COPY ./static /static
COPY ./templates /templates

# Expose the port your Actix Web app will run on
EXPOSE 8080

# Set the default command to run your application
CMD ["personal-website"]