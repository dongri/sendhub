FROM rust:1.38
LABEL maintainer "Dongri Jin <dongrify@gmail.com>"

ADD . /source
WORKDIR /source

EXPOSE 3000

RUN cargo build --release

CMD ["./target/release/sendhub"]
