# Use specific slim Rust version for reproducible builds and minimal vulnerabilities
FROM rust:1.89-slim AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo files and application code to the container
COPY . .

# Build the application in release mode
RUN cargo build --release

# Use distroless nonroot image for minimal attack surface and non-root execution
FROM gcr.io/distroless/cc-debian12:nonroot

# Set working directory for the application
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/personal-website /usr/local/bin/personal-website

# Copy application files with proper permissions for non-root user
COPY --chown=nonroot:nonroot ./posts ./posts
COPY --chown=nonroot:nonroot ./demos ./demos
COPY --chown=nonroot:nonroot ./publications ./publications
COPY --chown=nonroot:nonroot ./static ./static
COPY --chown=nonroot:nonroot ./templates ./templates

# Expose the port your Actix Web app will run on
EXPOSE 8080

# Set the default command to run your application
CMD ["personal-website"]