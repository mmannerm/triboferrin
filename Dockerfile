ARG TARGETARCH

FROM debian:bookworm-slim AS libs-amd64
RUN apt-get update && apt-get install -y libopus0 && rm -rf /var/lib/apt/lists/*
RUN mkdir -p /libs && cp /usr/lib/x86_64-linux-gnu/libopus.so.0* /libs/

FROM debian:bookworm-slim AS libs-arm64
RUN apt-get update && apt-get install -y libopus0 && rm -rf /var/lib/apt/lists/*
RUN mkdir -p /libs && cp /usr/lib/aarch64-linux-gnu/libopus.so.0* /libs/

FROM libs-${TARGETARCH} AS libs

###
FROM gcr.io/distroless/cc:nonroot

ARG TARGETARCH

WORKDIR /app
COPY triboferrin-linux-${TARGETARCH} /app/triboferrin
COPY --from=libs /libs/ /usr/lib/

ENV LD_LIBRARY_PATH=/usr/lib

USER nonroot
CMD ["/app/triboferrin"]
