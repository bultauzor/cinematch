import asyncio

import uvicorn
from fastapi import FastAPI, Request
from fastapi.middleware.cors import CORSMiddleware
from psycopg.errors import OperationalError

import routes
from db import init_db
from utils import expect_env_var

port = expect_env_var("PORT")

app = FastAPI()

# Cors ff
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

app.state.db = init_db()

app.include_router(routes.router)

@app.middleware("http")
async def db_check(request: Request, call_next):
    try:
        if request.app.state.db is None or request.app.state.db.closed:
            request.app.state.db.connect()
        # testing the connection
        with request.app.state.db.cursor() as cur:
            cur.execute("SELECT 1")
    except OperationalError as e:
        request.app.state.db.connect()

    response = await call_next(request)
    return response

async def main():
    config = uvicorn.Config(app, port=int(port), host="0.0.0.0")
    server = uvicorn.Server(config)
    await server.serve()


asyncio.run(main())
