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

`xml.xsd` (W3C) is shared by R4.4.0. Sync-guard: the
`schemas_match_pinned_py_armodel` test compares bytes against
`target/py-armodel/autosar/<release>/xsd/` when that checkout exists locally
(it is gitignored; CI skips the test).
