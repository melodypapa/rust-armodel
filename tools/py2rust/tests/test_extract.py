import sys, unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from extract import extract_source

SAMPLE = '''
from typing import List, Optional
from x import ARObject, AREnum, ARLiteral


class Modification(ARObject):
    """Records what has changed."""

    def __init__(self):
        super().__init__()
        self.change: Optional[MultiLanguageOverviewParagraph] = None


class Referrable(ARObject, ABC):
    """Can be referred to by its identifier."""

    def __init__(self, parent: ARObject, short_name: str):
        if type(self) is Referrable:
            raise TypeError("Referrable is an abstract class.")
        ARObject.__init__(self)
        self.parent: ARObject = parent
        self.short_name: str = short_name


class XmlSpaceEnum(AREnum):
    DEFAULT = "default"
    PRESERVE = "preserve"

    def __init__(self):
        super().__init__((XmlSpaceEnum.DEFAULT, XmlSpaceEnum.PRESERVE))


class NameToken(ARLiteral):
    pass
'''


class TestExtractSkeleton(unittest.TestCase):
    def setUp(self):
        self.ir = extract_source(SAMPLE, "armodel.models.M2.test")

    def test_classes_registered_with_bases(self):
        self.assertEqual(self.ir.get("Modification").bases, ["ARObject"])
        self.assertEqual(self.ir.get("Referrable").bases, ["ARObject"])  # ABC dropped

    def test_abstract_flag_from_typeerror_guard_and_abc_base(self):
        self.assertFalse(self.ir.get("Modification").is_abstract)
        self.assertTrue(self.ir.get("Referrable").is_abstract)

    def test_enum_and_primitive_flags(self):
        self.assertTrue(self.ir.get("XmlSpaceEnum").is_enum)
        self.assertFalse(self.ir.get("XmlSpaceEnum").is_abstract)
        self.assertTrue(self.ir.get("NameToken").is_primitive)

    def test_enum_literals_read_wire_values_with_doc(self):
        literals = {l.name: (l.value, l.doc) for l in self.ir.get("XmlSpaceEnum").enum_literals}
        self.assertEqual(literals["PRESERVE"][0], "preserve")


if __name__ == "__main__":
    unittest.main()
