FROM node:8-alpine AS build

# build deps
RUN yarn global add marked

WORKDIR /tmp/resume-build

COPY README.md .
RUN marked -gfm README.md -o index.html

FROM alpine:3.7

# runtime deps
RUN apk add --no-cache caddy

WORKDIR /app

COPY deploy/Caddyfile .
COPY --from=build /tmp/resume-build/index.html public/

CMD caddy
