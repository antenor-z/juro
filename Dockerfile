FROM rust:1.85

WORKDIR /juro
COPY . .

RUN cargo install --path .

EXPOSE 8000

CMD ["juro"]
