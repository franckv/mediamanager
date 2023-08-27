FROM rust:bookworm as builder

WORKDIR /usr/local/build

COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml api/
COPY model/Cargo.toml model/

RUN mkdir api/src && echo "fn main() {}" > ./api/src/main.rs && mkdir model/src && touch ./model/src/lib.rs && cargo build --release

COPY . .
RUN touch -a -m ./api/src/main.rs && touch -a -m ./model/src/lib.rs && cargo build --release

FROM debian:bookworm as base

RUN apt update && apt install -y udev eject curl libavcodec-extra libexpat1

FROM base as makemkv

WORKDIR /usr/local/build

COPY scripts/build_makemkv.sh .
RUN ./build_makemkv.sh

FROM base

COPY --from=builder /usr/local/build/target/release/mediamanager-api /bin/mediamanager-api
COPY config/default.conf /etc/mediamanager.conf
COPY scripts/mediamanager /bin/mediamanager
COPY scripts/udev/99-mediamanager.rules /etc/udev/rules.d/99-mediamanager.rules
COPY --from=makemkv /usr/lib/libdriveio.so.0 /usr/lib/libdriveio.so.0
COPY --from=makemkv /usr/lib/libmakemkv.so.1 /usr/lib/libmakemkv.so.1
COPY --from=makemkv /usr/lib/libmmbd.so.0 /usr/lib/libmmbd.so.0
COPY --from=makemkv /usr/bin/mmccextr /usr/bin/mmccextr
COPY --from=makemkv /usr/bin/mmgplsrv /usr/bin/mmgplsrv
COPY --from=makemkv /usr/bin/makemkvcon /usr/bin/makemkvcon

CMD ["/bin/mediamanager-api"]
