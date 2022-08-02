FROM node:16-alpine AS react
WORKDIR /app/
ADD *.js tsconfig.json package.json ./
ADD public ./public
ADD src-ui ./src-ui
RUN npm i && npm run build

FROM rust:alpine AS rust
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev
WORKDIR /app/
ADD Cargo.toml Cargo.lock build.rs ./
ADD src ./src
COPY --from=react /app/build ./build
RUN cargo build --release

FROM alpine
WORKDIR /app
COPY --from=rust /app/target/release/timezone_db .

CMD ["/app/timezone_db"]
