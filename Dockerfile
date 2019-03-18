FROM rust:latest

COPY . .

RUN cargo build

EXPOSE 80

CMD ./target/debug/dz1 -c /etc/httpd.conf -p 80 -a 0.0.0.0