from typing import Optional

from pydantic import BaseModel, Field, UUID4


class InputCredentials(BaseModel):
    username: str = Field(..., min_length=2, max_length=32, description='The username of the user')
    password: str = Field(..., min_length=4, max_length=128, description='The password of the user')


class Token(BaseModel):
    token: str = Field(..., description='The token of the user')


class UserInput(BaseModel):
    username: str = Field(..., min_length=2, max_length=32, description='The username of the user')
    password: str = Field(..., min_length=4, max_length=128, description='The password of the user')
    avatar: Optional[str] = Field(default=None, validate_default=False, max_length=4152444,
                                  description='The avatar of the user')


class User:
    user_id: UUID4
    username: str
    password: str
    avatar: Optional[str]

    def __init__(self, user_id: UUID4, username: str, password: str, avatar: Optional[str]):
        self.user_id = user_id
        self.username = username
        self.password = password
        self.avatar = avatar
