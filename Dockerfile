FROM --platform=$BUILDPLATFORM node:16-alpine AS REACT_BUILD
WORKDIR /app/
ADD package.json ./
RUN npm i
ADD *.js tsconfig.json ./
ADD public ./public
ADD src-ui ./src-ui
RUN npm run build

FROM --platform=$BUILDPLATFORM rust AS RUST_BUILD
WORKDIR /app/

# For openssl-sys (we're compiling openssl-src instead) w/ debian rust image
#RUN apt-get update && apt-get install -y \
#    musl-dev \
#    libssl-dev

# Compile ring using clang instead w/ debian rust image (this breaks compiling openssl-src)
#RUN apt-get install -y \
#    musl-tools \
#    clang \
#    llvm
#ENV CC_aarch64_unknown_linux_musl=clang
#ENV AR_aarch64_unknown_linux_musl=llvm-ar
#ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"
#ENV CC_x86_64_unknown_linux_musl=clang
#ENV AR_x86_64_unknown_linux_musl=llvm-ar
#ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"

# Setup musl cross compiler
# https://musl.cc/aarch64-linux-musl-cross.tgz & https://musl.cc/x86_64-linux-musl-cross.tgz
RUN cd /tmp && \
    curl -L https://github.com/xmake-mirror/musl.cc/releases/download/20210202/aarch64-linux-musl-cross.linux.tgz -o aarch64-linux-musl-cross.tgz && \
    curl -L https://github.com/xmake-mirror/musl.cc/releases/download/20210202/x86_64-linux-musl-cross.linux.tgz -o x86_64-linux-musl-cross.tgz && \
    tar -xzf aarch64-linux-musl-cross.tgz && \
    tar -xzf x86_64-linux-musl-cross.tgz
ENV CC_aarch64_unknown_linux_musl=/tmp/aarch64-linux-musl-cross/bin/aarch64-linux-musl-gcc
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"
ENV CC_x86_64_unknown_linux_musl=/tmp/x86_64-linux-musl-cross/bin/x86_64-linux-musl-gcc
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"

# Fetch cargo dependencies
ADD Cargo.toml Cargo.lock ./
RUN cargo fetch

# Setup target arch
ARG TARGETARCH
RUN rustup target add $(echo $TARGETARCH | sed 's/arm64/aarch64/;s/amd64/x86_64/')-unknown-linux-musl

ADD build.rs ./
ADD src ./src
COPY --from=REACT_BUILD /app/build ./build

RUN cargo build --release --target $(echo $TARGETARCH | sed 's/arm64/aarch64/;s/amd64/x86_64/')-unknown-linux-musl
RUN mv /app/target/$(echo $TARGETARCH | sed 's/arm64/aarch64/;s/amd64/x86_64/')-unknown-linux-musl/release/timezone_db .

FROM --platform=$TARGETPLATFORM alpine
WORKDIR /app
EXPOSE 8000
COPY --from=RUST_BUILD /app/timezone_db .
CMD ["/app/timezone_db"]
