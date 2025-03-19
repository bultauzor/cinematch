import os


# get an env var, if the env var is not provided the program crash
def expect_env_var(var: str) -> str:
    res = os.environ.get(var)
    if res is None:
        print("error: please provide env", var)
        exit(1)
    return res
