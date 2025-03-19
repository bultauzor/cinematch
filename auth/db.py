from typing import Optional

import psycopg
from psycopg import Connection
from pydantic import UUID4

from models import User
from utils import expect_env_var


def init_db():
    pg_db = expect_env_var("POSTGRESQL_ADDON_DB")
    pg_host = expect_env_var("POSTGRESQL_ADDON_HOST")
    pg_password = expect_env_var("POSTGRESQL_ADDON_PASSWORD")
    pg_port = expect_env_var("POSTGRESQL_ADDON_PORT")
    pg_user = expect_env_var("POSTGRESQL_ADDON_USER")

    return psycopg.connect(
        dbname=pg_db, host=pg_host, password=pg_password, port=pg_port, user=pg_user
    )


def get_user(db: Connection, username: str) -> Optional[User]:
    cursor = db.cursor()
    cursor.execute(
        'SELECT user_id, username, password FROM users WHERE username = %s', (username,)
    )
    res = cursor.fetchone()
    if res is not None:
        return User(
            user_id=res[0],
            username=res[1],
            password=res[2],
            avatar=None
        )
    else:
        return None


def update_user_password(db: Connection, user_id: UUID4, password: str) -> None:
    cursor = db.cursor()
    cursor.execute('UPDATE users SET password = %s WHERE user_id = %s', (password, user_id))
    db.commit()
    cursor.close()


def insert_user(db: Connection, user: User) -> None:
    cursor = db.cursor()
    cursor.execute('INSERT INTO users (user_id, username, password, avatar) VALUES (%s, %s, %s, %s)',
                   (user.user_id, user.username, user.password, user.avatar))
    db.commit()
    cursor.close()
