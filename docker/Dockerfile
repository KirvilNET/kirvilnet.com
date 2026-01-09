FROM ubuntu:22.04

COPY ./app /app

WORKDIR /app

RUN chmod +x ./kirvilnet-server

EXPOSE 3000

ENTRYPOINT ["./kirvilnet-server"]