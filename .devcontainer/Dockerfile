ARG UBUNTU_VERSION=26.04
FROM ubuntu:${UBUNTU_VERSION}

# Upstream versions; apt package revisions may vary within these releases.
ARG TASK_VERSION=3.54.0
ARG RUST_VERSION=1.100.0
ARG GO_VERSION=1.27.3
ARG GOPLS_VERSION=v0.24.0
ARG GOLANGCI_LINT_VERSION=v2.15.0
ARG ZIG_VERSION=0.17.1
ARG ZLS_VERSION=0.17.0
ARG LLVM_VERSION=23
ARG ELIXIR_VERSION=1.18.3
ARG ERLANG_VERSION=27.3.4
ARG HEX_VERSION=2.7.0
ARG LUA_VERSION=5.4.8
ARG LUACHECK_VERSION=1.2.0
ARG STYLUA_VERSION=2.6.0
ARG LUALS_VERSION=3.19.2
ARG DART_VERSION=3.14.0
ARG PYTHON_VERSION=3.15.2
ARG RUFF_VERSION=0.17.3
ARG MYPY_VERSION=2.4.1

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

ARG DEBIAN_FRONTEND=noninteractive
ENV RUSTUP_HOME=/usr/local/rustup \
    PATH="/usr/local/cargo/bin:/usr/local/go/bin:/usr/local/zig:/usr/local/lib/lua-language-server/bin:/usr/lib/dart/bin:${PATH}"

# System updates and common tools.
RUN apt-get update && apt-get upgrade -y \
    && apt-get install -y --no-install-recommends \
        bash build-essential ca-certificates curl git software-properties-common \
        sudo tar unzip xz-utils openssh-client bubblewrap \
    && rm -rf /var/lib/apt/lists/*

# Match the host user's UID through Dev Containers, keeping checkout files writable.
RUN (id ubuntu >/dev/null 2>&1 || useradd --create-home --uid 1000 --shell /bin/bash ubuntu) \
    && echo 'ubuntu ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/ubuntu \
    && chmod 0440 /etc/sudoers.d/ubuntu

RUN curl -fsSL https://taskfile.dev/install.sh -o /tmp/install-task.sh \
    && sh /tmp/install-task.sh -b /usr/local/bin "v${TASK_VERSION}" \
    && rm /tmp/install-task.sh

# Rust: compiler, standard library sources, formatter and Clippy.
RUN curl -fsSL https://sh.rustup.rs -o /tmp/install-rust.sh \
    && CARGO_HOME=/usr/local/cargo sh /tmp/install-rust.sh -y --no-modify-path \
        --default-toolchain "${RUST_VERSION}" --profile minimal --component rust-src,rustfmt,clippy \
    && rm /tmp/install-rust.sh

# Go: compiler, gofmt, vet, test and editor tools.
RUN curl -fsSL "https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz" \
        -o /tmp/go.tar.gz \
    && tar -xzf /tmp/go.tar.gz -C /usr/local \
    && rm /tmp/go.tar.gz

RUN GOBIN=/usr/local/bin go install "golang.org/x/tools/gopls@${GOPLS_VERSION}" \
    && go clean -modcache -cache
RUN curl -fsSL https://golangci-lint.run/install.sh -o /tmp/install-golangci-lint.sh \
    && sh /tmp/install-golangci-lint.sh -b /usr/local/bin "${GOLANGCI_LINT_VERSION}" \
    && rm /tmp/install-golangci-lint.sh

# Zig: compiler and formatter.
RUN curl -fsSL "https://ziglang.org/download/${ZIG_VERSION}/zig-x86_64-linux-${ZIG_VERSION}.tar.xz" \
        -o /tmp/zig.tar.xz \
    && mkdir /usr/local/zig \
    && tar -xJf /tmp/zig.tar.xz -C /usr/local/zig --strip-components=1 \
    && rm /tmp/zig.tar.xz

RUN curl -fsSL "https://builds.zigtools.org/zls-x86_64-linux-${ZLS_VERSION}.tar.xz" \
        -o /tmp/zls.tar.xz \
    && tar -xJf /tmp/zls.tar.xz -C /usr/local/bin zls \
    && rm /tmp/zls.tar.xz

# C++23: current stable LLVM compiler, standard library, editor and check tools.
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
    && apt-get install -y --no-install-recommends \
        "elixir=${ELIXIR_VERSION}*" "erlang-base=1:${ERLANG_VERSION}*" \
        "erlang-dev=1:${ERLANG_VERSION}*" "erlang-dialyzer=1:${ERLANG_VERSION}*" \
        "erlang-src=1:${ERLANG_VERSION}*" \
    && rm -rf /var/lib/apt/lists/* \
    && sudo -H -u ubuntu mix local.hex "${HEX_VERSION}" --force

RUN curl -fsSL "https://github.com/elixir-lang/elixir/archive/refs/tags/v${ELIXIR_VERSION}.tar.gz" \
        -o /tmp/elixir-src.tar.gz \
    && mkdir -p /usr/local/src/elixir \
    && tar -xzf /tmp/elixir-src.tar.gz -C /usr/local/src/elixir --strip-components=1 \
    && rm /tmp/elixir-src.tar.gz

# Lua interpreter, linter, formatter and standalone language server.
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        "lua${LUA_VERSION%.*}=${LUA_VERSION}*" "lua-check=${LUACHECK_VERSION}*" \
    && rm -rf /var/lib/apt/lists/*
RUN lua_series=${LUA_VERSION%.*} \
    && CARGO_HOME=/usr/local/cargo cargo install stylua --version "${STYLUA_VERSION}" \
        --locked --no-default-features --features "lua${lua_series//./}" --root /usr/local
RUN curl -fsSL "https://github.com/LuaLS/lua-language-server/releases/download/${LUALS_VERSION}/lua-language-server-${LUALS_VERSION}-linux-x64.tar.gz" \
        -o /tmp/luals.tar.gz \
    && mkdir -p /usr/local/lib/lua-language-server \
    && tar -xzf /tmp/luals.tar.gz -C /usr/local/lib/lua-language-server \
    && rm /tmp/luals.tar.gz

# Dart: stable SDK, including the compiler, formatter, analyser and language server.
RUN curl -fsSL https://dl-ssl.google.com/linux/linux_signing_key.pub \
        -o /usr/share/keyrings/dart.asc \
    && echo "deb [signed-by=/usr/share/keyrings/dart.asc arch=amd64] https://storage.googleapis.com/download.dartlang.org/linux/debian stable main" \
        > /etc/apt/sources.list.d/dart.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends "dart=${DART_VERSION}*" \
    && sudo -H -u ubuntu dart --disable-analytics \
    && rm -rf /var/lib/apt/lists/*

# Python interpreter and development tools
RUN add-apt-repository -y ppa:deadsnakes/ppa \
    && apt-get update \
    && apt-get install -y --no-install-recommends python3 \
        "python${PYTHON_VERSION%.*}=${PYTHON_VERSION}*" \
    && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL https://bootstrap.pypa.io/get-pip.py -o /tmp/get-pip.py \
    && "python${PYTHON_VERSION%.*}" /tmp/get-pip.py --break-system-packages \
    && "python${PYTHON_VERSION%.*}" -m pip install --break-system-packages --ignore-installed --no-cache-dir \
        "ruff==${RUFF_VERSION}" "mypy==${MYPY_VERSION}" \
    && rm /tmp/get-pip.py

USER ubuntu
