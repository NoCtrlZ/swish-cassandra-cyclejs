FROM alpine:3.12

COPY . .

RUN apk add --update --upgrade \
    rust=1.44.0-r0 \
    cargo=1.44.0-r0

EXPOSE 3000

ENTRYPOINT [ "cargo", "run" ]
