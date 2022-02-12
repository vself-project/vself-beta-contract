# Use the official lightweight Node.js 14 image.
# https://hub.docker.com/_/node
FROM node:14-slim

# Create and change to the app directory.
WORKDIR /usr/src/app

# Copy application dependency manifests to the container image.
# A wildcard is used to ensure copying both package.json AND package-lock.json (when available).
# Copying this first prevents re-running npm install on every code change.
COPY package*.json ./
COPY yarn.lock ./

# Install production dependencies.
# If you add a package-lock.json, speed your build by switching to 'npm ci'.
# RUN npm ci --only=production
RUN npm install --only=production

# Copy local code to the container image.
COPY server.js ./
RUN mkdir -p ./config
COPY config/near.js ./config/

# Run the web service on container startup.
RUN mkdir -p ./creds
COPY creds/beta_v4.ilerik.testnet.json ./creds
CMD [ "node", "server.js" ]