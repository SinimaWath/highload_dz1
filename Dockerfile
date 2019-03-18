FROM rust:latest

COPY httpd.conf /etc/httpd.conf
COPY . .

RUN cargo build --release

EXPOSE 80

CMD ./target/debug/dz1 -c /etc/httpd.conf -p 80 -a 0.0.0.0