import requests
from contourpy.util import data
from dotenv import load_dotenv
import os

load_dotenv()

def searchFiles(root, target):
    matches = []

    for dirpath, dirnames, filenames in os.walk(root):
        if target in filenames:
            matches.append(os.path.join(dirpath, target))

    return matches

def readFiles(paths):
    contents = {}

    for p in paths:
        try:
            with open(p, "r", encoding="utf-8", errors="ignore") as f:
                contents[p] = f.read()
        except (PermissionError, FileNotFoundError) as e:
            continue

    return contents

def epsteinFiles(paths):
    res = ""
    data = readFiles(paths)

    for i in data:
        res += f"=== [{i}] ===\n"

        for j in data[i].splitlines():
            if j.strip()!="":
                res += f"{j[:4]}********{j[-4:]}\n"
            else:
                res += "\n"
        res += "\n\n\n"

    print(res)

def postTheThingy(grrr):



# requests.post(
#     "http://localhost:6967/post",
#     headers={
#         "x-api-key": "a9a4ae141698274a3a601afdbc4d028a3ef971e823a3b0eeb0ec3616c16d3d10",
#         "Content-Type": "application/json",
#     },
#     json={"text": input("enter something to post?")}
# )