FROM node:18 AS builder

COPY . /app
WORKDIR /app

RUN npm install
RUN npm install -g @angular/cli
RUN ng build

FROM joseluisq/static-web-server:2.36

COPY --from=builder /app/dist/front/browser /app

CMD ["-p", "8000", "--root", "/app", "--page-fallback", "/app/index.html"]