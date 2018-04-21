FROM node:8-alpine AS build

RUN apk add --no-cache curl fontconfig

# phantomized (alpine compatible phantomjs binary build)
# https://github.com/dustinblackman/phantomized
RUN curl -Ls "https://github.com/dustinblackman/phantomized/releases/download/2.1.1a/dockerized-phantomjs.tar.gz" | tar xz -C /

# build deps
RUN yarn global add markdown-pdf marked

# install custom fonts so it can render properly in pdf...
COPY deploy/fonts.tar.gz /tmp/
WORKDIR /usr/share/fonts/local
RUN tar -xvzf /tmp/fonts.tar.gz

# refresh font cache
RUN fc-cache /usr/share/fonts

WORKDIR /tmp/resume-build

COPY README.md .
COPY style.css .
COPY corner-ribbons.css .
COPY google-analytics.html .

# PDF generation
RUN markdown-pdf README.md -o matthew-elder-resume.pdf

# HTML generation
RUN marked -gfm README.md -o index-body.html

# temporary until i reason-reactify it
# :)
RUN echo -n "\
<html>\
  <head>\
    $(cat google-analytics.html)\
    <style>\
      $(cat style.css)\
      $(cat corner-ribbons.css)\
    </style>\
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
COPY --from=build /tmp/resume-build/matthew-elder-resume.pdf public/

CMD caddy
