FROM rust:latest as builder

WORKDIR /usr/local/build

COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml api/
COPY model/Cargo.toml model/

RUN mkdir api/src && echo "fn main() {}" > ./api/src/main.rs && mkdir model/src && touch ./model/src/lib.rs && cargo build --release

COPY . .
RUN touch -a -m ./api/src/main.rs && touch -a -m ./model/src/lib.rs && cargo build --release

FROM debian:bullseye as base

RUN apt update && apt install -y udev eject wget
RUN wget https://apt.benthetechguy.net/benthetechguy-archive-keyring.gpg -O /usr/share/keyrings/benthetechguy-archive-keyring.gpg
RUN echo "deb [signed-by=/usr/share/keyrings/benthetechguy-archive-keyring.gpg] https://apt.benthetechguy.net/debian bullseye main contrib non-free" > /etc/apt/sources.list.d/benthetechguy.list
RUN apt update && apt install -y makemkvcon

FROM base

COPY --from=builder /usr/local/build/target/release/mediamanager-api /bin/mediamanager-api
COPY config/mediamanager.conf /etc/mediamanager.conf

CMD ["/bin/mediamanager-api"]
