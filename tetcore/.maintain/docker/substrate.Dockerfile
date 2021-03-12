FROM debian:stretch-slim

# metadata
ARG VCS_REF
ARG BUILD_DATE

LABEL io.tetsy.image.authors="devops-team@parity.io" \
	io.tetsy.image.vendor="Tetsy Technologies" \
	io.tetsy.image.title="tetcoin/tetcore" \
	io.tetsy.image.description="Tetcore: The platform for blockchain innovators." \
	io.tetsy.image.source="https://github.com/tetcoin/tetcore/blob/${VCS_REF}/.maintain/docker/Dockerfile" \
	io.tetsy.image.revision="${VCS_REF}" \
	io.tetsy.image.created="${BUILD_DATE}" \
	io.tetsy.image.documentation="https://wiki.tetcoin.org/Tetcoin-Tetcore"

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get upgrade -y && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y \
		libssl1.1 \
		ca-certificates \
		curl && \
# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete; \
# add user
	useradd -m -u 1000 -U -s /bin/sh -d /tetcore tetcore

# add tetcore binary to docker image
COPY ./tetcore /usr/local/bin

USER tetcore

# check if executable works in this container
RUN /usr/local/bin/tetcore --version

EXPOSE 30333 9933 9944
VOLUME ["/tetcore"]

ENTRYPOINT ["/usr/local/bin/tetcore"]

