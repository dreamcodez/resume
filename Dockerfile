FROM node:8-alpine AS build

# build deps
RUN yarn global add marked

WORKDIR /tmp/resume-build

COPY README.md .
COPY style.css style.css
RUN marked -gfm README.md -o index-body.html

# temporary until i reason-reactify it
# :)
RUN echo -n "\
<html>\
  <head>\
    <style>$(cat style.css)</style>\
  </head>\
  <body>$(cat index-body.html)</body>\
</html>\
" > index.html

FROM alpine:3.7

# runtime deps
RUN apk add --no-cache caddy

WORKDIR /app

COPY deploy/Caddyfile .
COPY --from=build /tmp/resume-build/index.html public/

CMD caddy
