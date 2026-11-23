"""Unabhängiger STEP-Rundlauf des Referenzzylinders mit Open CASCADE."""
import math
from pathlib import Path
import sys
import tempfile

from OCP.BRep import BRep_Tool
from OCP.BRepAdaptor import BRepAdaptor_Surface
from OCP.BRepCheck import BRepCheck_Analyzer
from OCP.BRepGProp import BRepGProp
from OCP.GeomAbs import GeomAbs_Cylinder, GeomAbs_Plane
from OCP.GProp import GProp_GProps
from OCP.IFSelect import IFSelect_RetDone
from OCP.STEPControl import STEPControl_AsIs, STEPControl_Reader, STEPControl_Writer
from OCP.TopAbs import TopAbs_FACE, TopAbs_SHELL, TopAbs_SOLID
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS


def parts(shape, kind):
    explorer = TopExp_Explorer(shape, kind)
    while explorer.More():
        yield explorer.Current()
        explorer.Next()


def check(path):
    reader = STEPControl_Reader()
    assert reader.ReadFile(str(path)) == IFSelect_RetDone
    assert reader.TransferRoots() == 1
    shape = reader.OneShape()
    assert len(list(parts(shape, TopAbs_SOLID))) == 1
    shell, = parts(shape, TopAbs_SHELL)
    assert BRep_Tool.IsClosed_s(shell) and BRepCheck_Analyzer(shape).IsValid()
    faces = [BRepAdaptor_Surface(TopoDS.Face_s(f)) for f in parts(shape, TopAbs_FACE)]
    assert len(faces) == 3
    side, = [f.Cylinder() for f in faces if f.GetType() == GeomAbs_Cylinder]
    axis = side.Axis().Direction().Coord()
    expected = (0.5, 0., math.sqrt(3.) / 2)
    assert math.dist(axis, expected) < 1e-8
    assert abs(side.Radius() - 7.) < 1e-7
    ends = [f.Plane().Location().Coord() for f in faces if f.GetType() == GeomAbs_Plane]
    assert len(ends) == 2
    ends.sort(key=lambda p: sum(a*b for a,b in zip(p, expected)))
    assert math.dist(ends[0], (100., 120., 9.)) < 1e-7
    assert abs(math.dist(*ends) - 19.) < 1e-7
    props = GProp_GProps()
    BRepGProp.VolumeProperties_s(shape, props)
    assert abs(props.Mass() - math.pi * 7**2 * 19) < 1e-5
    return shape


if __name__ == "__main__":
    original = check(Path(sys.argv[1]))
    with tempfile.TemporaryDirectory() as directory:
        target = Path(directory) / "Rundlauf.step"
        writer = STEPControl_Writer()
        assert writer.Transfer(original, STEPControl_AsIs) == IFSelect_RetDone
        assert writer.Write(str(target)) == IFSelect_RetDone
        check(target)
    print("Beide STEP-Dateien geprüft: geschlossener Zylinder, drei Flächen, Radius 7 mm, Höhe 19 mm, Position und 30° Neigung.")
