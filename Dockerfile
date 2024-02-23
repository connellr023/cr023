FROM rust:latest

WORKDIR /usr/src/app
COPY . .
RUN apt-get update && \
    apt-get install -y libssl-dev
EXPOSE 8080
CMD ["trunk", "serve"]