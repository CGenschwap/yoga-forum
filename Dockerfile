FROM rust:1.60 as builder

WORKDIR /usr/src/app
COPY yoga-forum/ yoga-forum/

WORKDIR /usr/src/app/yoga-forum
RUN cargo install --path .

# TODO: Slimify
FROM rust:1.60
#FROM debian:buster-slim
#
#RUN apt-get update \
#    && apt-get install -y \
#        ca-certificates \
#        gcc \
#        libc6-dev \
#    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/yoga-forum /usr/local/bin/yoga-forum

WORKDIR /usr/src/app

COPY public/ public/
COPY favicon/ favicon/

CMD ["/usr/local/bin/yoga-forum"]
