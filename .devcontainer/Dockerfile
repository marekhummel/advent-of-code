FROM ubuntu:26.04

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

ARG DEBIAN_FRONTEND=noninteractive

# System updates and common tools.
RUN apt-get update && apt-get upgrade -y \
    && apt-get install -y --no-install-recommends \
        bash build-essential ca-certificates curl git software-properties-common \
        sudo tar unzip xz-utils openssh-client bubblewrap \
    && rm -rf /var/lib/apt/lists/*

ARG TASK_VERSION=3.54.0
RUN curl -fsSL https://taskfile.dev/install.sh -o /tmp/install-task.sh \
    && sh /tmp/install-task.sh -b /usr/local/bin "v${TASK_VERSION}" \
    && rm /tmp/install-task.sh

# Rust: compiler, standard library sources, formatter and Clippy.
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH="/usr/local/cargo/bin:/usr/local/go/bin:/usr/local/zig:${PATH}"
RUN curl -fsSL https://sh.rustup.rs -o /tmp/install-rust.sh \
    && CARGO_HOME=/usr/local/cargo sh /tmp/install-rust.sh -y --no-modify-path \
        --default-toolchain stable --profile minimal --component rust-src,rustfmt,clippy \
    && rm /tmp/install-rust.sh

# Go: compiler, gofmt, vet, test and editor tools.
ARG TARGETARCH
ARG GO_VERSION=1.27.3
RUN curl -fsSL "https://go.dev/dl/go${GO_VERSION}.linux-${TARGETARCH}.tar.gz" \
        -o /tmp/go.tar.gz \
    && tar -xzf /tmp/go.tar.gz -C /usr/local \
    && rm /tmp/go.tar.gz

ARG GOPLS_VERSION=v0.24.0
ARG GOLANGCI_LINT_VERSION=v2.15.0
RUN GOBIN=/usr/local/bin go install golang.org/x/tools/gopls@${GOPLS_VERSION} \
    && go clean -modcache -cache
RUN curl -fsSL https://golangci-lint.run/install.sh -o /tmp/install-golangci-lint.sh \
    && sh /tmp/install-golangci-lint.sh -b /usr/local/bin "${GOLANGCI_LINT_VERSION}" \
    && rm /tmp/install-golangci-lint.sh

# Zig: compiler and formatter.
ARG ZIG_VERSION=0.17.1
RUN case "$TARGETARCH" in amd64) zig_arch=x86_64 ;; arm64) zig_arch=aarch64 ;; \
        *) echo "Unsupported Zig architecture: $TARGETARCH" >&2; exit 1 ;; esac \
    && curl -fsSL "https://ziglang.org/download/${ZIG_VERSION}/zig-${zig_arch}-linux-${ZIG_VERSION}.tar.xz" \
        -o /tmp/zig.tar.xz \
    && mkdir /usr/local/zig \
    && tar -xJf /tmp/zig.tar.xz -C /usr/local/zig --strip-components=1 \
    && rm /tmp/zig.tar.xz

# C++23: current stable LLVM compiler, standard library, editor and check tools.
ARG LLVM_VERSION=23
RUN curl -fsSL https://apt.llvm.org/llvm-snapshot.gpg.key \
        -o /usr/share/keyrings/llvm.asc \
    && echo "deb [signed-by=/usr/share/keyrings/llvm.asc] https://apt.llvm.org/resolute/ llvm-toolchain-resolute-${LLVM_VERSION} main" \
        > /etc/apt/sources.list.d/llvm.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends \
        clang-${LLVM_VERSION} libc++-${LLVM_VERSION}-dev libc++abi-${LLVM_VERSION}-dev \
        clangd-${LLVM_VERSION} clang-format-${LLVM_VERSION} clang-tidy-${LLVM_VERSION} lldb-${LLVM_VERSION} \
    && for tool in clang clang++ clangd clang-format clang-tidy lldb; do \
        ln -s "/usr/bin/${tool}-${LLVM_VERSION}" "/usr/local/bin/${tool}"; \
       done \
    && rm -rf /var/lib/apt/lists/*

# Elixir, Erlang/OTP and Mix from Ubuntu's repositories.
RUN apt-get update \
    && apt-get install -y --no-install-recommends elixir erlang-dev erlang-dialyzer \
    && rm -rf /var/lib/apt/lists/*

# Python interpreter and development tools
RUN add-apt-repository -y ppa:deadsnakes/ppa \
    && apt-get update \
    && apt-get install -y --no-install-recommends python3 python3.15 \
    && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL https://bootstrap.pypa.io/get-pip.py -o /tmp/get-pip.py \
    && python3.15 /tmp/get-pip.py --break-system-packages \
    && python3.15 -m pip install --break-system-packages --ignore-installed --no-cache-dir \
        ruff==0.17.3 mypy==2.4.1 \
    && rm /tmp/get-pip.py

# Match the host user's UID through Dev Containers, keeping checkout files writable.
RUN (id ubuntu >/dev/null 2>&1 || useradd --create-home --uid 1000 --shell /bin/bash ubuntu) \
    && echo 'ubuntu ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/ubuntu \
    && chmod 0440 /etc/sudoers.d/ubuntu

USER ubuntu

# Hex fetches the development dependency used by the Elixir check task.
RUN mix local.hex --force
