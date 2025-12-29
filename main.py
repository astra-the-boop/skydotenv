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

    return res

def postTheThingy(grrr):
    requests.post(
        "http://localhost:6967/post",
        headers={
            "x-api-key": "a9a4ae141698274a3a601afdbc4d028a3ef971e823a3b0eeb0ec3616c16d3d10",
            "Content-Type": "application/json",
        },
        json={"text": grrr}
    )

if input("Hey are you really sure you wanna do this? [y/n]").lower() == "y":
    if input("You know that this will post your env files publicly for millions to see right? [yes/n]").lower() == "yes":
        if input("This is really important stuff! Please make sure you know this [I understand/n]") == "I understand":
            if input("Last warning! [I understand and fully consent to a censored version of the contents of all the files with the name '.env' to be sent and posted publicly online on the social media site Bluesky under the handle @skydotenv.bsky.social/n]") == "I understand and fully consent to a censored version of the contents of all the files with the name '.env' to be sent and posted publicly online on the social media site Bluesky under the handle @skydotenv.bsky.social":
                postTheThingy(epsteinFiles(searchFiles("/Users/astra.celestine/Desktop/skydotenv", ".env")))