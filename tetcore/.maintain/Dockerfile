# Note: We don't use Alpine and its packaged Rust/Cargo because they're too often out of date,
# preventing them from being used to build Tetcore/Polkadot.

FROM phusion/baseimage:0.11 as builder
LABEL maintainer="chevdor@gmail.com"
LABEL description="This is the build stage for Tetcore. Here we create the binary."

ENV DEBIAN_FRONTEND=noninteractive

ARG PROFILE=release
WORKDIR /tetcore

COPY . /tetcore

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
	export PATH="$PATH:$HOME/.cargo/bin" && \
	rustup toolchain install nightly && \
	rustup target add wasm32-unknown-unknown --toolchain nightly && \
	rustup default stable && \
	cargo build "--$PROFILE"

# ===== SECOND STAGE ======

FROM phusion/baseimage:0.11
LABEL maintainer="chevdor@gmail.com"
LABEL description="This is the 2nd stage: a very small image where we copy the Tetcore binary."
ARG PROFILE=release

RUN mv /usr/share/ca* /tmp && \
	rm -rf /usr/share/*  && \
	mv /tmp/ca-certificates /usr/share/ && \
	useradd -m -u 1000 -U -s /bin/sh -d /tetcore tetcore && \
	mkdir -p /tetcore/.local/share/tetcore && \
	chown -R tetcore:tetcore /tetcore/.local && \
	ln -s /tetcore/.local/share/tetcore /data

COPY --from=builder /tetcore/target/$PROFILE/tetcore /usr/local/bin
COPY --from=builder /tetcore/target/$PROFILE/tetkey /usr/local/bin
COPY --from=builder /tetcore/target/$PROFILE/node-rpc-client /usr/local/bin
COPY --from=builder /tetcore/target/$PROFILE/node-template /usr/local/bin
COPY --from=builder /tetcore/target/$PROFILE/chain-spec-builder /usr/local/bin

# checks
RUN ldd /usr/local/bin/tetcore && \
	/usr/local/bin/tetcore --version

# Shrinking
RUN rm -rf /usr/lib/python* && \
	rm -rf /usr/bin /usr/sbin /usr/share/man

USER tetcore
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

CMD ["/usr/local/bin/tetcore"]
