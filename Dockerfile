FROM ekidd/rust-musl-builder:1.51.0 as builder

ARG BUILD_TARGET=x86_64-unknown-linux-musl

WORKDIR /app

COPY ./ ./

RUN sudo chown -R rust:rust .
RUN cargo build --release

RUN cp ./target/BUILD_TARGET/release/libretest /app/libretest

FROM alpine:latest as libretest

# Install libpq for postgres
ENV HOST 0.0.0.0
ENV PORT 8080
ENV DATABASE_URL postgres://libretest:password@localhost/libretest
ENV REDIS_HOST localhost
ENV REDIS_PORT 6379

RUN apk add libpq

RUN addgroup -g 1000 libretest
RUN adduser -D -s /bin/sh -u 1000 -G libretest libretest

# Copy resources
COPY --chown=libretest:libretest --from=builder /app/libretest /app/libretest

RUN chown libretest:libretest /app/libretest
USER libretest
EXPOSE 8080
CMD ["/app/libretest"]