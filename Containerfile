FROM node
COPY ./frontend /build
WORKDIR /build
RUN npm ci
RUN npm run build

FROM rust
COPY . /build
WORKDIR /build
COPY --from=0 /build/build/ /build/frontend/build
RUN cargo build --release

FROM fedora
COPY --from=1 /build/target/release/vereinsverwaltung /
WORKDIR /data
CMD ../vereinsverwaltung
