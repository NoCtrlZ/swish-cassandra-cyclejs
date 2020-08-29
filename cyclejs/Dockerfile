FROM node:12.11.1-alpine

WORKDIR /app

COPY package.json .

RUN yarn

EXPOSE 8080

ENTRYPOINT [ "yarn", "start" ]
