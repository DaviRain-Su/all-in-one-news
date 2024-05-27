# we use the latest Rust stable release as base image
FROM rust:1.78.0 AS builder
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y
# Copy all files from our working environment to our Docker image
COPY . .
ENV SQLX_OFFLINE true
# Let's build our binary!
# We'll use the release profile to make it faaaast
RUN cargo build --release

# runtime stage
FROM debian:bullseye-slim AS runtime

WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
#RUN apt-get update -y \
#    && apt-get install -y --no-install-recommends openssl ca-certificates \
#    # Clean up
#    && apt-get autoremove -y \
#    && apt-get clean -y \
#    && rm -rf /var/lib/apt/lists/*

# Install build dependencies and utilities
RUN apt-get update -y && apt-get install -y --no-install-recommends \
    build-essential \
    wget \
    ca-certificates

# Download and install OpenSSL 3.0 from source
RUN wget https://www.openssl.org/source/openssl-3.0.0.tar.gz \
    && tar -zxf openssl-3.0.0.tar.gz \
    && cd openssl-3.0.0 \
    && ./config --prefix=/usr/local/ssl --openssldir=/usr/local/ssl shared zlib \
    && make \
    && make install

# Setup environment to use installed OpenSSL
ENV LD_LIBRARY_PATH=/usr/local/ssl/lib
ENV PATH=$PATH:/usr/local/ssl/bin

# Clean up unnecessary packages and files
RUN apt-get autoremove -y build-essential wget \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/* /openssl-3.0.0.tar.gz /openssl-3.0.0

COPY --from=builder /app/target/release/aion aion
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./aion"]
