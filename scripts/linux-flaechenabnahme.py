"""Fensterabnahme mit X11: Keilschale per Mausklick öffnen, speichern und zurücknehmen."""
import csv
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]


def run(*args):
    return subprocess.check_output(args)


def xdo(*args):
    return run("xdotool", *map(str, args))


def click(window, x, y):
    xdo("mousemove", "--window", window, x, y, "click", 1)
    time.sleep(0.4)


def button(window, word, width, height, down=False):
    # Die tatsächliche Lage des beschrifteten Bedienelements aus dem Fenster lesen.
    for attempt in range(12):
        image = run("import", "-window", window, "png:-")
        tsv = subprocess.run(
            ["tesseract", "stdin", "stdout", "-l", "deu", "--psm", "11", "tsv"],
            input=image, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, check=True,
        ).stdout.decode()
        rows = list(csv.DictReader(io.StringIO(tsv), delimiter="\t"))
        for row in rows:
            if (row["text"].casefold().strip(".,:") in (("3d", "wählen") if word == "3d" else ("übernehmen","ubernehmen")) and
                    int(row["left"]) > width * 0.72):
                click(window, int(row["left"]) + int(row["width"]) // 2,
                      int(row["top"]) + int(row["height"]) // 2)
                return
        if attempt > 0:
            xdo("mousemove", "--window", window, width - 65, height // 2,
                "click", "--repeat", 3, 5 if down else 4)
        time.sleep(0.3)
    raise AssertionError(f"Schaltfläche {word} fehlt: " + " ".join(r["text"] for r in rows))


def reference(path):
    obj = json.loads(path.read_text())["objects"][0]
    return obj["feature"]["Shell"]["side"]["Plane"]


def save(window, path):
    before = path.stat().st_mtime_ns
    xdo("key", "ctrl+s")
    for _ in range(50):
        if path.stat().st_mtime_ns != before:
            return reference(path)
        time.sleep(0.1)
    raise AssertionError("Speicherung im Fenster fehlt")


def main():
    with tempfile.TemporaryDirectory() as temp:
        path = Path(temp) / "Prüfung Keilschale.r3dp"
        shutil.copyfile(ROOT / "examples/modelle/Keilschale.r3dp", path)
        original = reference(path)
        env = dict(os.environ, XDG_DATA_HOME=temp, XDG_CONFIG_HOME=temp)
        app = subprocess.Popen([Path(sys.argv[1]).resolve(), path], cwd=temp, env=env)
        try:
            window = subprocess.check_output(
                ["xdotool", "search", "--sync", "--onlyvisible",
                 "--name", "^Rust 3D Print Studio$"], timeout=30,
            ).decode().splitlines()[0]
            xdo("windowfocus", "--sync", window)
            geometry = dict(line.split("=") for line in xdo(
                "getwindowgeometry", "--shell", window).decode().splitlines())
            w, h = int(geometry["WIDTH"]), int(geometry["HEIGHT"])
            time.sleep(1)
            xdo("key", "ctrl+a")
            time.sleep(0.4)
            button(window, "3d", w, h)
            time.sleep(1)
            click(window, w // 2, h // 2 + 16)
            button(window, "übernehmen", w, h, down=True)
            time.sleep(1)
            bound = save(window, path)
            assert bound != original and abs(bound["normal"][0] - 2**-0.5) < 1e-6, bound
            xdo("key", "ctrl+z")
            time.sleep(0.3)
            assert save(window, path) == original
            xdo("key", "ctrl+y")
            time.sleep(0.3)
            assert save(window, path) == bound
            print("Schräge angeklickt, übernommen, gespeichert, zurückgenommen und wiederhergestellt.")
        finally:
            app.terminate()
            app.wait(timeout=10)


if __name__ == "__main__":
    main()
