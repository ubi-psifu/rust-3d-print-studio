"""Unabhängiger STEP-Rundlauf des Referenzzylinders mit Open CASCADE."""
import math
from pathlib import Path
import sys
import tempfile

from OCP.BRep import BRep_Tool
from OCP.BRepClass3d import BRepClass3d_SolidClassifier
from OCP.gp import gp_Pnt
from OCP.TopAbs import TopAbs_IN, TopAbs_OUT
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


def check(path, tube):
    reader = STEPControl_Reader()
    assert reader.ReadFile(str(path)) == IFSelect_RetDone
    assert reader.TransferRoots() == 1
    shape = reader.OneShape()
    assert len(list(parts(shape, TopAbs_SOLID))) == 1
    shell, = parts(shape, TopAbs_SHELL)
    assert BRep_Tool.IsClosed_s(shell) and BRepCheck_Analyzer(shape).IsValid()
    faces = [BRepAdaptor_Surface(TopoDS.Face_s(f)) for f in parts(shape, TopAbs_FACE)]
    assert len(faces) == (4 if tube else 3)
    sides = sorted((f.Cylinder() for f in faces if f.GetType() == GeomAbs_Cylinder),
                   key=lambda side: side.Radius())
    radii = [4., 7.] if tube else [7.]
    assert len(sides) == len(radii)
    expected = (0.5, 0., math.sqrt(3.) / 2)
    for side, radius in zip(sides, radii):
        assert math.dist(side.Axis().Direction().Coord(), expected) < 1e-8
        assert abs(side.Radius() - radius) < 1e-7
        assert side.Axis().Location().Distance(sides[-1].Axis().Location()) < 1e-7
    ends = [f.Plane().Location().Coord() for f in faces if f.GetType() == GeomAbs_Plane]
    assert len(ends) == 2
    ends.sort(key=lambda p: sum(a*b for a,b in zip(p, expected)))
    assert math.dist(ends[0], (100., 120., 9.)) < 1e-7
    assert abs(math.dist(*ends) - 19.) < 1e-7
    solid, = parts(shape, TopAbs_SOLID)
    for height in (0.1, 9.5, 18.9):
        center = [v + height * a for v, a in zip(ends[0], expected)]
        for offset, state in [(0., TopAbs_OUT if tube else TopAbs_IN), (5., TopAbs_IN), (8., TopAbs_OUT)]:
            x, y, z = center
            point = gp_Pnt(x, y + offset, z)
            assert BRepClass3d_SolidClassifier(solid, point, 1e-7).State() == state
    props = GProp_GProps()
    BRepGProp.VolumeProperties_s(shape, props)
    assert abs(props.Mass() - math.pi * (49 - (16 if tube else 0)) * 19) < 1e-5
    return shape


if __name__ == "__main__":
    tube = sys.argv[2:] == ["--rohr"]
    original = check(Path(sys.argv[1]), tube)
    with tempfile.TemporaryDirectory() as directory:
        target = Path(directory) / "Rundlauf.step"
        writer = STEPControl_Writer()
        assert writer.Transfer(original, STEPControl_AsIs) == IFSelect_RetDone
        assert writer.Write(str(target)) == IFSelect_RetDone
        check(target, tube)
    print("STEP-Rundlauf bestanden:", "Rohr mit 8 mm Durchgangsbohrung" if tube else "Vollzylinder")
