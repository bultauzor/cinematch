import asyncio

import uvicorn
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

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


async def main():
    config = uvicorn.Config(app, port=int(port), host="0.0.0.0")
    server = uvicorn.Server(config)
    await server.serve()


asyncio.run(main())
