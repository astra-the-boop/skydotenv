import time
import os
import requests
import webbrowser
import sys
from tqdm import tqdm


def searchFiles(root, target):
    matches = []

    with tqdm(desc="Scanning files", unit="", dynamic_ncols=True, leave=False, smoothing=0.1, bar_format="{l_bar}{bar}| {n_fmt} [{rate_fmt}] {postfix}") as thingy:
        for dirpath, dirnames, filenames in os.walk(root):
            thingy.set_postfix_str(dirpath, refresh=False)

            if target in filenames:
                matches.append(os.path.join(dirpath, target))

            thingy.update(1)

    return matches

def readFiles(paths):
    contents = {}

    for p in paths:
        try:
            with open(p, "r", encoding="utf-8", errors="ignore") as f:
                contents[p] = f.read()
        except (PermissionError, FileNotFoundError):
            continue

    return contents

def epsteinFiles(paths):
    res = ""
    data = readFiles(paths)

    for i in tqdm(data, desc="Redacting files...", unit="file"):
        res += f"=== [{i}] ===\n"

        for j in data[i].splitlines():
            if j.strip()!="":
                res += f"{j[:4]}********{j[-4:]}\n"
            else:
                res += "\n"
        res += "\n\n\n"

    return res

def postTheThingy(grrr):
    res = requests.post(
        "http://mackerel-moved-elephant.ngrok-free.app/post",
        headers={
            "x-api-key": "a9a4ae141698274a3a601afdbc4d028a3ef971e823a3b0eeb0ec3616c16d3d10",
            "Content-Type": "application/json",
        },
        json={"text": grrr}
    )

    res.raise_for_status()
    data = res.json()

    return data["url"]


def clear():
    if os.name=="nt":
        os.system("cls")
    else:
        os.system("clear")

def main():
    clear()
    if input("Hey are you really sure you wanna do this? [y/n]").lower() == "y":
        clear()
        if input("You know that this will post your env files publicly for millions to see right? [yes/n]").lower() == "yes":
            clear()
            if input("This is really important stuff! Please make sure you know this [I understand/n]") == "I understand":
                clear()
                if input("Last warning! [I understand and fully consent to a censored version of the contents of all the files with the name '.env' to be sent and posted publicly online on the social media site Bluesky under the handle @skydotenv.bsky.social/n]") == "I understand and fully consent to a censored version of the contents of all the files with the name '.env' to be sent and posted publicly online on the social media site Bluesky under the handle @skydotenv.bsky.social":
                    clear()
                    print("You can always stop this by pressing Ctrl+C you know...")
                    time.sleep(1)
                    for i in range(5):
                        print(5-i)
                        time.sleep(1)
                    clear()
                    time.sleep(1.5)
                    print("Scanning and posting...")
                    url = postTheThingy(epsteinFiles(searchFiles("/", ".env")))
                    webbrowser.open(url)
                    clear()
                    input(f"thanks for that, i guess? you can check out your env files here: {url}")
    clear()
    print("Please quit the application")
    sys.exit(0)



if __name__ == "__main__":
    main()