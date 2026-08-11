FROM mcr.microsoft.com/devcontainers/base:bookworm

ARG DEBIAN_FRONTEND=noninteractive
ARG NODE_MAJOR=24

ENV CARGO_HOME=/usr/local/cargo \
	RUSTUP_HOME=/usr/local/rustup \
	PATH=/usr/local/cargo/bin:${PATH}

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

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal \
	&& cargo install wasm-pack cargo-llvm-cov cargo-audit cargo-deny

RUN curl -LsSf https://astral.sh/uv/install.sh | sh

RUN mkdir -p /home/vscode/.cargo \
	&& chown -R vscode:vscode /home/vscode/.cargo /usr/local/rustup

ENV CARGO_HOME=/home/vscode/.cargo \
	PATH=/home/vscode/.cargo/bin:/usr/local/cargo/bin:${PATH}

USER vscode
