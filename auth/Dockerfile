FROM python:3.13-alpine

RUN apk add curl libpq libpq-dev

COPY . /app
WORKDIR /app

RUN pip install -r requirements.txt

ENTRYPOINT ["python", "__main__.py"]