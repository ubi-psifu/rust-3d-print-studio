"""Unabhängige Prüfung der drei benannten Teile in STEP-Platte.r3dp."""
import math
import sys

from OCP.BRepCheck import BRepCheck_Analyzer
from OCP.BRepGProp import BRepGProp
from OCP.GProp import GProp_GProps
from OCP.IFSelect import IFSelect_RetDone
from OCP.STEPCAFControl import STEPCAFControl_Reader
from OCP.TCollection import TCollection_ExtendedString
from OCP.TDataStd import TDataStd_Name
from OCP.TDF import TDF_LabelSequence
from OCP.TDocStd import TDocStd_Document
from OCP.TopAbs import TopAbs_SOLID
from OCP.XCAFDoc import XCAFDoc_DocumentTool

expected = {
    "Führung A": ((30, 40, 9), 0, math.pi * 49 * 19),
    "Rohr 'B'": ((110, 90, 9), 30, math.pi * 33 * 19),
    "Block #3": ((180, 160, 9), -30, (24 * 16 - math.pi * 16) * 19),
}

reader = STEPCAFControl_Reader()
doc = TDocStd_Document(TCollection_ExtendedString("Abnahme"))
assert reader.ReadFile(sys.argv[1]) == IFSelect_RetDone and reader.Transfer(doc)
shapes = XCAFDoc_DocumentTool.ShapeTool_s(doc.Main())
labels = TDF_LabelSequence()
shapes.GetFreeShapes(labels)
assert labels.Length() == len(expected), labels.Length()
seen = set()
for index in range(1, labels.Length() + 1):
    label = labels.Value(index)
    attribute = TDataStd_Name()
    assert label.FindAttribute(TDataStd_Name.GetID_s(), attribute)
    name = attribute.Get().ToExtString()
    assert name in expected and name not in seen, name
    seen.add(name)
    base, tilt, volume = expected[name]
    shape = shapes.GetShape_s(label)
    assert shape.ShapeType() == TopAbs_SOLID and BRepCheck_Analyzer(shape).IsValid()
    props = GProp_GProps()
    BRepGProp.VolumeProperties_s(shape, props)
    angle = math.radians(tilt)
    center = (base[0] + 9.5 * math.sin(angle), base[1], base[2] + 9.5 * math.cos(angle))
    assert math.dist(props.CentreOfMass().Coord(), center) < 1e-7, name
    assert abs(props.Mass() - volume) < 1e-5, name
print("Drei benannte, einzeln lesbare Volumenkörper mit Sollvolumen und Plattenlage bestätigt.")
