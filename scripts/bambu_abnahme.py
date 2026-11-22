"""Prüft eine reale P1S-Vorlage mit mindestens zwei Filamenten über den installierten Slicer."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import xml.etree.ElementTree as ET
from zipfile import ZipFile

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("bambu", type=Path)
parser.add_argument("vorlage", type=Path)
args = parser.parse_args()
root = Path(__file__).resolve().parents[1]
with tempfile.TemporaryDirectory() as directory:
    work = Path(directory)
    source = work / "Platte.3mf"
    result = work / "Rueckgabe.3mf"
    subprocess.run(["cargo", "run", "--locked", "--example", "slicer_abnahme", "--",
                    str(args.vorlage.resolve()), str(source)], cwd=root, check=True)
    subprocess.run([str(args.bambu.resolve()), "--arrange", "0", "--orient", "0",
                    "--export-3mf", str(result), str(source)], cwd=work, check=True)
    with ZipFile(result) as archive:
        settings = json.loads(archive.read("Metadata/project_settings.config"))
        for key, expected in {"printer_model": "Bambu Lab P1S", "layer_height": "0.16",
                              "wall_loops": "4", "sparse_infill_density": "27%",
                              "enable_support": "1"}.items():
            assert settings[key] == expected, key
        with ZipFile(args.vorlage) as original:
            template = json.loads(original.read("Metadata/project_settings.config"))
        assert settings["filament_settings_id"] == template["filament_settings_id"]
        model = ET.fromstring(archive.read("3D/3dmodel.model"))
        poses = {v.get("objectid"): tuple(map(float, v.attrib["transform"].split()[-3:]))
                 for v in model.findall(".//{*}item")}
        objects = ET.fromstring(archive.read("Metadata/model_settings.config"))
        actual = {}
        for obj in objects.findall("object"):
            values = {m.get("key"): m.get("value") for m in obj.findall("metadata")}
            actual[values["name"]] = (values["extruder"], poses[obj.attrib["id"]])
        assert actual == {"Teil & Sockel": ("1", (40, 50, 10)), "Gegenstück": ("2", (202, 191, 10))}
        assert len(objects.findall("plate/model_instance")) == 2
    print("Geprüft: zwei Positionen, Namen, Filamente, Plattenzuordnung und vier Prozesswerte.")
