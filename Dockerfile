FROM rust:1.77
WORKDIR /usr/src/app
EXPOSE 8000
COPY . .

RUN cargo install --path .

CMD ["transparent-ai-webserver"]
