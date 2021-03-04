FROM debian:stretch-slim

# metadata
ARG VCS_REF
ARG BUILD_DATE

LABEL io.tetsy.image.authors="devops-team@parity.io" \
	io.tetsy.image.vendor="Tetsy Technologies" \
	io.tetsy.image.title="tetsy/tetkey" \
	io.tetsy.image.description="Subkey: key generating utility for Tetcore." \
	io.tetsy.image.source="https://github.com/tetcoin/tetcore/blob/${VCS_REF}/.maintain/docker/tetkey.Dockerfile" \
	io.tetsy.image.revision="${VCS_REF}" \
	io.tetsy.image.created="${BUILD_DATE}" \
	io.tetsy.image.documentation="https://github.com/tetcoin/tetcore/tree/${VCS_REF}/tetkey"

# show backtraces
ENV RUST_BACKTRACE 1

# add user
RUN useradd -m -u 1000 -U -s /bin/sh -d /tetkey tetkey

# add tetkey binary to docker image
COPY ./tetkey /usr/local/bin

USER tetkey

# check if executable works in this container
RUN /usr/local/bin/tetkey --version

ENTRYPOINT ["/usr/local/bin/tetkey"]

