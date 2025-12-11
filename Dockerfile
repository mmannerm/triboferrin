FROM debian:bookworm-slim AS libs

RUN apt-get update && apt-get install -y \
    libopus0 \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /libs && \
    cp /usr/lib/$(dpkg-architecture -qDEB_HOST_MULTIARCH)/libopus.so.0* /libs/

###
FROM gcr.io/distroless/cc:nonroot

ARG TARGETARCH

WORKDIR /app
COPY triboferrin-linux-${TARGETARCH} /app/triboferrin
COPY --from=libs /libs/ /usr/lib/

ENV LD_LIBRARY_PATH=/usr/lib

USER nonroot
CMD ["/app/triboferrin"]
