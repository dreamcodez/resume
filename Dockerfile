FROM --platform=linux/amd64 node:12.18.2-alpine3.12 AS toolchain


FROM toolchain AS built

WORKDIR /opt/app

# Install NPM Dependencies
COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile

# Build App Runtime
COPY rollup.config.js svelte.config.js tsconfig.json ./
COPY src/ src/
COPY static/ static/
COPY typings/ typings/
RUN yarn build

FROM --platform=linux/amd64 node:12.18.2-alpine3.12 AS final

WORKDIR /opt/app

# Install only production NPM Dependencies
COPY --from=built /opt/app/package.json /opt/app/yarn.lock ./
RUN yarn install --production --frozen-lockfile

# Copy built js and static resources
COPY --from=built /opt/app/__sapper__/ __sapper__/
COPY --from=built /opt/app/static/ static/

ENTRYPOINT [ "node", "__sapper__/build" ]