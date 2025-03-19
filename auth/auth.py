import uuid
from typing import Optional

from argon2 import PasswordHasher
from argon2.exceptions import VerifyMismatchError
from biscuit_auth.biscuit_auth import BiscuitBuilder, PrivateKey
from fastapi import HTTPException
from psycopg import Connection

import db
from models import User, InputCredentials, UserInput
from utils import expect_env_var

hasher = PasswordHasher()
pk = PrivateKey.from_hex(expect_env_var("PRIVATE_KEY"))


def check_credentials(conn: Connection, credentials: InputCredentials) -> Optional[User]:
    user = db.get_user(conn, credentials.username)
    if user is None:
        print(f"User {credentials.username} not found")
        return None

    try:
        hasher.verify(user.password, credentials.password)
    except VerifyMismatchError:
        print(f"Password mismatch for user {credentials.username}")
        return None

    db.update_user_password(conn, user.user_id, hasher.hash(credentials.password))
    return user


def generate_token(user: User) -> str:
    bb = BiscuitBuilder("""
    user({user_id});
    username({username});
    """, {
        "user_id": user.user_id.__str__(),
        "username": user.username,
    })
    biscuit = bb.build(pk)
    return biscuit.to_base64()


def build_new_user(conn: Connection, input: UserInput) -> User:
    if db.get_user(conn, input.username) is not None:
        raise HTTPException(400, detail=f"User {input.username} already exists")

    user = User(user_id=uuid.uuid4(), username=input.username, password=hasher.hash(input.password),
                avatar=input.avatar)

    db.insert_user(conn, user)
    return user
