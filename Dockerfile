FROM rustlang/rust:nightly-alpine as builder

RUN apk add musl-dev

WORKDIR /build

COPY . .

RUN cargo build --bin skolaonline-ical-aas --release

FROM scratch

WORKDIR /app

COPY --from=builder /build/target/release/skolaonline-ical-aas .

ENTRYPOINT ["/app/skolaonline-ical-aas"]
