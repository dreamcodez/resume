FROM alpine:3.6

WORKDIR /app

RUN apk add --no-cache curl
COPY .allmark .

RUN \
  curl http://allmark.io/bin/files/allmark_linux_amd64 -o /usr/local/bin/allmark &&\
  chmod a+x /usr/local/bin/allmark

CMD \
  curl -sO https://raw.githubusercontent.com/dreamcodez/resume/master/README.md &&\
  allmark serve
