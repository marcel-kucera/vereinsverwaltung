# Build frontend
FROM --platform=$BUILDPLATFORM node as frontend-build
COPY ./frontend /frontend
WORKDIR /frontend
RUN npm ci
RUN npm run build

FROM --platform=$BUILDPLATFORM rust as backend-build
RUN apt update
RUN apt -y install gcc-aarch64-linux-gnu
RUN rustup target add aarch64-unknown-linux-gnu
COPY . /backend
WORKDIR /backend
COPY --from=frontend-build /frontend/build/ /backend/frontend/build
RUN cargo build --release --target aarch64-unknown-linux-gnu

FROM --platform=arm64 debian
LABEL org.opencontainers.image.source="https://github.com/marcel-kucera/vereinsverwaltung"
COPY --from=backend-build /backend/target/aarch64-unknown-linux-gnu/release/vereinsverwaltung /
WORKDIR /data
CMD ../vereinsverwaltung
