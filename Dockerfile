FROM rust:1.85

WORKDIR /juro
COPY . .

RUN cargo install --path .

CMD ["juro"]
