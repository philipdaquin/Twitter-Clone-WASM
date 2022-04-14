FROM node:14.7.0-alpine

WORKDIR ./apollo-gateway
RUN apk add --no-cache bash
COPY package.json index.js wait-for-it.sh ./
RUN npm install
RUN chmod +x wait-for-it.sh
# wait until all downstream services start
CMD ./wait-for-it.sh blog-service:8080 -s -- \
    ./wait-for-it.sh auth-service:8081 -s -- \
    npm run start-gateway
EXPOSE 4000