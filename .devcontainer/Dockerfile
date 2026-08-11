FROM mcr.microsoft.com/devcontainers/base:bookworm

ARG DEBIAN_FRONTEND=noninteractive
ARG NODE_MAJOR=24

RUN apt-get update \
	&& apt-get install -y --no-install-recommends \
		ca-certificates \
		curl \
		git \
		gnupg \
	&& rm -rf /var/lib/apt/lists/*

RUN curl -fsSL https://deb.nodesource.com/setup_${NODE_MAJOR}.x | bash - \
	&& apt-get install -y --no-install-recommends nodejs \
	&& rm -rf /var/lib/apt/lists/*

RUN export RUSTUP_HOME=/usr/local/rustup CARGO_HOME=/usr/local/cargo \
	&& curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal \
	&& "$CARGO_HOME/bin/cargo" install wasm-pack cargo-llvm-cov cargo-audit cargo-deny

RUN export UV_INSTALL_DIR=/usr/local/bin \
	&& curl -LsSf https://astral.sh/uv/install.sh | sh

RUN mkdir -p /home/vscode/.cargo \
	&& cp -a /usr/local/cargo/. /home/vscode/.cargo/ \
	&& chown -R vscode:vscode /home/vscode/.cargo /usr/local/rustup

ENV CARGO_HOME=/home/vscode/.cargo \
	RUSTUP_HOME=/usr/local/rustup \
	PATH=/home/vscode/.cargo/bin:/usr/local/cargo/bin:/usr/local/bin:${PATH}

USER vscode
