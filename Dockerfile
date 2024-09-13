# Use a multi-stage build for smaller final image
FROM rust:1.70 AS builder

WORKDIR /capital_gains
# Copy your source tree
COPY . .

# Build for release with the specified target
RUN cargo build --release

# Final stage
FROM debian:buster-slim

# Copy the build artifact from the build stage
# The exact path depends on the target, so we use a wildcard
COPY --from=builder /capital_gains/target/release/capital_gains /capital_gains

# Set the startup command to run your binary
CMD ["/capital_gains"]