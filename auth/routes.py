from fastapi import APIRouter, Request, HTTPException

import auth
from models import InputCredentials, Token, UserInput

router = APIRouter()


@router.get("/ping")
def ping() -> str:
    """
    Ping the API
    """
    return "PONG!"


@router.post("/auth")
async def authenticate(req: Request, creds: InputCredentials) -> Token:
    """
    Authenticate
    """
    user = auth.check_credentials(req.app.state.db, creds)
    if user is None:
        raise HTTPException(status_code=401, detail="Incorrect username or password")

    token = auth.generate_token(user)
    return Token(token=token)


@router.post("/register")
async def register(req: Request, input: UserInput) -> Token:
    """
    Register
    """
    user = auth.build_new_user(req.app.state.db, input)
    token = auth.generate_token(user)
    return Token(token=token)
