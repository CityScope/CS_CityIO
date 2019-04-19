
# configure file for golang
#FROM golang:1.12
#
#WORKDIR /go/src/app
#COPY . .
#
#RUN go get -d -v ./...
#RUN go install -v ./...
#RUN go build
#
#CMD ["app"]



# configure file for rust

FROM rust:1.34

WORKDIR /usr/src/myapp
COPY . .

#RUN cargo install --path .
#RUN cargo build
#RUN cargo build --release
#RUN cargo install --path .
EXPOSE 8080
CMD ["cargo","run"]
#CMD ["cs_cityio_backend"]

