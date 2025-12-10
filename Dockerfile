FROM rust:latest AS builder

RUN apt-get update && apt-get install -y \
    cmake \
    libopus-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . /app

RUN cargo build --release

# Determine the library path based on architecture and copy to a known location
RUN mkdir -p /app/libs && \
    cp /usr/lib/$(dpkg-architecture -qDEB_HOST_MULTIARCH)/libopus.so.0* /app/libs/

###
FROM gcr.io/distroless/cc:nonroot

WORKDIR /app
COPY --from=builder /app/target/release/triboferrin /app
COPY --from=builder /app/libs/ /usr/lib/

ENV LD_LIBRARY_PATH=/usr/lib

USER nonroot
CMD ["/app/triboferrin"]