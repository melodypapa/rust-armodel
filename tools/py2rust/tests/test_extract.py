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


FIELD_SAMPLE = '''
from typing import List, Optional
from x import ARObject


class DocRevision(ARObject):
    def __init__(self):
        super().__init__()
        # This specifies the date and time. Tags: xml.sequenceOffset=80
        self.date: Optional[DateTime] = None
        self.issuedBy: Optional[String] = None
        self.modifications: List[Modification] = []


class AdminData(ARObject):
    def __init__(self):
        super().__init__()
        self.DocRevisions: List[DocRevision] = []
        self.language: Optional[LEnum] = None
        self.sdgs: List[Sdg] = []
        self.usedLanguages: Optional[MultiLanguagePlainText] = None

    def getUsedLanguages(self) -> Optional[MultiLanguagePlainText]:
        """This property specifies the languages. Tags: xml.sequenceOffset=30"""
        return self.usedLanguages


class SdgCaption(ARObject):
    def __init__(self, parent, short_name):
        super().__init__(parent, short_name)
        self.desc: Optional[MultiLanguageOverviewParagraph] = None


class VariationPointCapable(ABC):
    variationPoint: Optional[VariationPoint] = None

    def getVariationPoint(self) -> Optional[VariationPoint]:
        return self.variationPoint


class ARPackage(ARObject):
    def __init__(self):
        super().__init__()
        self.arPackages: List[ARPackage] = []

    def createARPackage(self, short_name: str) -> ARPackage:
        ar_package = ARPackage(self, short_name)
        self.arPackages.append(ar_package)
        return ar_package

    def createApplicationSwComponentType(self, short_name: str) -> ApplicationSwComponentType:
        sw_component = ApplicationSwComponentType(self, short_name)
        self.addElement(sw_component)
        return sw_component
'''


class TestExtractFields(unittest.TestCase):
    def setUp(self):
        self.ir = extract_source(FIELD_SAMPLE, "armodel.models.M2.test")

    def test_optional_and_list_fields(self):
        doc_revision = self.ir.get("DocRevision")
        by_name = {f.name: f for f in doc_revision.fields}
        self.assertEqual(by_name["date"].kind, "optional")
        self.assertEqual(by_name["date"].inner, "DateTime")
        self.assertEqual(by_name["modifications"].kind, "list")
        self.assertEqual(by_name["modifications"].inner, "Modification")
        self.assertIn("xml.sequenceOffset=80", by_name["date"].doc)

    def test_getter_docstring_wins_over_comment(self):
        admin = {f.name: f for f in self.ir.get("AdminData").fields}
        self.assertIn("xml.sequenceOffset=30", admin["usedLanguages"].doc)

    def test_constructor_params_parent_short_name_are_not_fields(self):
        self.assertEqual([f.name for f in self.ir.get("SdgCaption").fields], ["desc"])

    def test_class_level_mixin_fields(self):
        mixin = self.ir.get("VariationPointCapable")
        self.assertEqual([f.name for f in mixin.fields], ["variationPoint"])
        self.assertEqual(mixin.fields[0].kind, "optional")

    def test_create_factories_collected_by_name(self):
        creates = self.ir.get("ARPackage").creates
        self.assertEqual(creates["createARPackage"], "ARPackage")
        self.assertEqual(creates["createApplicationSwComponentType"], "ApplicationSwComponentType")


if __name__ == "__main__":
    unittest.main()
