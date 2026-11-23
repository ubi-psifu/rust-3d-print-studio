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


def button(window, word, width, height, down=False, toolbar=False):
    # Die tatsächliche Lage des beschrifteten Bedienelements aus dem Fenster lesen.
    for attempt in range(12):
        left = 0 if toolbar else int(width * 0.72)
        region_height = 70 if toolbar else height
        image = run("import", "-window", window, "-crop", f"{width-left}x{region_height}+{left}+0",
                    "+repage", "-resize", "250%", "-colorspace", "Gray", "-negate", "png:-")
        tsv = subprocess.run(
            ["tesseract", "stdin", "stdout", "-l", "deu", "--psm", "11", "tsv"],
            input=image, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, check=True,
        ).stdout.decode()
        rows = list(csv.DictReader(io.StringIO(tsv), delimiter="\t"))
        for row in rows:
            labels = {"3d": ("3d", "wählen"), "übernehmen": ("übernehmen", "ubernehmen", "parameter")}
            if row["text"].casefold().strip(".,:") in labels.get(word, (word,)):
                click(window, left + round((int(row["left"]) + int(row["width"]) / 2) / 2.5),
                      round((int(row["top"]) + int(row["height"]) / 2) / 2.5))
                return
        if attempt > 0:
            xdo("mousemove", "--window", window, width - 65, height // 2,
                "click", "--repeat", 3, 5 if down else 4)
        time.sleep(0.3)
    raise AssertionError(f"Schaltfläche {word} fehlt: " + " ".join(r["text"] for r in rows))


def reference(path):
    obj = json.loads(path.read_text())["objects"][0]
    if "EdgeFinish" in obj["feature"]:
        return obj["feature"]["EdgeFinish"].get("reference")
    return obj["feature"]["Shell"]["side"]["Plane"]


def save(window, path):
    before = path.stat().st_mtime_ns
    xdo("key", "ctrl+s")
    for _ in range(50):
        if path.stat().st_mtime_ns != before:
            return reference(path)
        time.sleep(0.1)
    raise AssertionError("Speicherung im Fenster fehlt")


def check_model(model):
    with tempfile.TemporaryDirectory() as temp:
        path = Path(temp) / "Prüfung Keilschale.r3dp"
        shutil.copyfile(ROOT / "examples/modelle" / (model + ".r3dp"), path)
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
            offset = round(0.18 * (h - 25)) - 4 if model == "Kantenquader" else 0
            click(window, w // 2 + offset, h // 2 + 16)
            button(window, "vorschau", w, h)
            toggle = "schnitt" if model == "Keilschale" else "bisherigen"
            if model == "Kantenquader":
                button(window, toggle, w, h, toolbar=True)
            def pixels():
                return run("import", "-window", window, "-crop", f"{w//2}x{h//2}+{w//4}+{h//4}",
                           "+repage", "-depth", "8", "rgb:-")
            before = pixels()
            button(window, toggle, w, h, toolbar=True)
            after = pixels()
            assert after != before, "Vergleich muss das gerenderte Modell wechseln"
            if model == "Keilschale":
                def cut_color_count(image):
                    return sum(r > 150 and r > g > b * 2 for r, g, b in zip(image[::3], image[1::3], image[2::3]))
                assert cut_color_count(before) > cut_color_count(after) + 50, "Schnittfläche fehlt"
            xdo("key", "Escape")
            time.sleep(0.5)
            assert save(window, path) == original, "Vorschau hat den Projektstand verändert"
            button(window, "vorschau", w, h)
            button(window, "ergebnis", w, h, toolbar=True)
            time.sleep(1)
            bound = save(window, path)
            assert json.loads(path.read_text())["objects"][0]["position"] == [128.,128.,0.]
            assert bound and bound != original, bound
            if model == "Keilschale":
                assert abs(bound["normal"][0] - 2**-0.5) < 1e-6
            else:
                assert all(abs(p[0] - 1.) < 1e-6 and abs(p[2] - 1.) < 1e-6 for p in bound["ends"]), bound
            xdo("key", "ctrl+z")
            time.sleep(0.3)
            assert save(window, path) == original
            xdo("key", "ctrl+y")
            time.sleep(0.3)
            assert save(window, path) == bound
            print(model + " angeklickt, übernommen, gespeichert, zurückgenommen und wiederhergestellt.")
        finally:
            app.terminate()
            app.wait(timeout=10)


if __name__ == "__main__":
    for model in ("Keilschale", "Kantenquader"):
        check_model(model)
