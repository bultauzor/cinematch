FROM node:18 AS builder

ARG API_URL

COPY . /app
WORKDIR /app

RUN sed -i "s,http://prod-url,$API_URL,g" /app/src/environments/environment.ts

RUN npm install
RUN npm install -g @angular/cli
RUN ng build

FROM joseluisq/static-web-server:2.36

COPY --from=builder /app/dist/front/browser /app

CMD ["-p", "8000", "--root", "/app", "--page-fallback", "/app/index.html"]