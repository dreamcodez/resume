FROM python:3.6-alpine3.6


RUN apk add --no-cache curl
RUN pip install --no-cache-dir grip

CMD curl https://raw.githubusercontent.com/dreamcodez/resume/master/README.md | grip -
