FROM rust:1.85.1

RUN cargo install sqlx-cli --no-default-features --features postgres

COPY . /app
WORKDIR /app

ENTRYPOINT ["sh", "-c", "sqlx migrate run -D postgresql://$POSTGRESQL_ADDON_USER:$POSTGRESQL_ADDON_PASSWORD@$POSTGRESQL_ADDON_HOST:$POSTGRESQL_ADDON_PORT/$POSTGRESQL_ADDON_DB"]