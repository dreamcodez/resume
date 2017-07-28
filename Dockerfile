FROM alpine:3.6

WORKDIR /app

RUN \
  apk add --no-cache curl && \
  curl -s http://allmark.io/bin/files/allmark_linux_amd64 -o /usr/local/bin/allmark && \
  chmod a+x /usr/local/bin/allmark && \
  apk del --no-cache curl

COPY .allmark/ .allmark/

CMD allmark serve
