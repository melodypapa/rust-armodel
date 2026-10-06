# Bundled AUTOSAR schemas

Byte-copies of the XSD sets vendored by the pinned py-armodel checkout
(`tools/py2rust/PY_ARMODEL_VERSION` pin; source: autosar.org release tarballs,
see py-armodel `autosar/<release>/xsd/`). Registries:

| File (case-insensitive) | Release |
|---|---|
| `AUTOSAR_00052.xsd` | R23-11 |
| `AUTOSAR_00046.xsd` | R4.4.0 |
| `AUTOSAR_00044.xsd` | R4.3.1 |
| `AUTOSAR.xsd`       | R3.2.3 |

`xml.xsd` (W3C) ships with R4.4.0 and is copied into R23-11/ and R4.3.1/ —
their top-level schemas import it from their own directory (libxml2 resolves
includes relative to the schema file). Sync-guard: the
`schemas_match_pinned_py_armodel` test compares bytes against
`target/py-armodel/autosar/<release>/xsd/` when that checkout exists locally
(it is gitignored; CI skips the test); `xml_xsd_copies_match_the_shared_original`
keeps the copies identical to the R4.4.0 original.
