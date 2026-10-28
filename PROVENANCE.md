# Provenance

## Source checkpoint

- Repository: `https://github.com/anreonyr/sqware.git`
- Public pull request: **#44**
- Frozen Source HEAD: `1854d607648ceacfa1d998004478c47c3793700c`
- Frozen commit subject: `Consume the published Hub API contracts`
- Source branch: `codex/shuyra-hub-api-extraction`
- Commit author: `anreonyr <anreonyr@gmail.com>`
- IOMMU subtrees: `programs/src/driver/iommu/declaration`,
  `programs/src/driver/iommu/api`, `programs/src/driver/iommu/hold`
- Test subtrees: `programs/tests/iommu-pages`,
  `programs/tests/retention`
- Toolchain copied from Source: `nightly-2026-09-04`, edition 2024, resolver 3.

The Source working tree was clean at this commit before and after this
snapshot. This preparation wrote no Source file, deleted no Source file, and
made no Source commit.

This repository is a contract-only snapshot of declarations, API, and the
reference-holding ledger. It does not claim a standalone Driver Unit, server,
runtime, or image build.

## Authors

Git history for the selected IOMMU and test subtrees records these author
identities:

- `anreonyr <anreonyr@gmail.com>`
- `anreonyr <anreonyr@users.noreply.github.com>`

The original history remains at `anreonyr/sqware`; this repository is a fresh
snapshot, not a history rewrite.

## License

No `LICENSE`, `COPYING`, or `NOTICE` file is tracked at the frozen Source
commit. The Source `Cargo.toml` and the selected package manifests declare
neither `license` nor `authors`. This snapshot therefore adds no license and
asserts no copyright permission. Do not infer a license from publication or
from this document.

## Production move and hashes

The Source declaration, API, and hold files are copied byte-for-byte. There is
no second or substitute `UnitFile`; `declaration/src/program.rs` is the only
one, and its program name remains `iommu`. The private `Memory` record remains
in its own file, `wire::step` remains private, and the generated `Fail` table is
unchanged.

| Source file | Provider file | SHA-256 (source = provider) |
| --- | --- | --- |
| `programs/src/driver/iommu/declaration/Cargo.toml` | `iommu/declaration/Cargo.toml` | `0711849bcdcd6b3ad66d6f65f3c98e0c67f349caf31ba3afc0c9fb6a17d04c84` |
| `programs/src/driver/iommu/declaration/src/lib.rs` | `iommu/declaration/src/lib.rs` | `1ce42950c908241132e5a651efdf9ac6bd312e8136c869e2dcb27516ac3ed2c6` |
| `programs/src/driver/iommu/declaration/src/bindings.rs` | `iommu/declaration/src/bindings.rs` | `b9d90cf54be8977ddf2e89df5a36ae81119d176fc05b09ea5efc83d7819cd378` |
| `programs/src/driver/iommu/declaration/src/marks.rs` | `iommu/declaration/src/marks.rs` | `a5b8e0ad204410df19ff447a668ad7d00f7bde27c1c9a86f4e088a564b1067bd` |
| `programs/src/driver/iommu/declaration/src/program.rs` | `iommu/declaration/src/program.rs` | `15e5bce1bc7dd9ed9535e57e550f7b3c83fceb1ae9ef8c4d84e652187fa6e20d` |
| `programs/src/driver/iommu/api/Cargo.toml` | `iommu/api/Cargo.toml` | `2fff5a5d913338b26e987a485a4bb401c18b86b3c9444cc681ffca5076922f8a` |
| `programs/src/driver/iommu/api/src/lib.rs` | `iommu/api/src/lib.rs` | `c53422c23be5872fb7bddd221c7ab4b64855b265749c96539f1d95dbb66f9130` |
| `programs/src/driver/iommu/api/src/acquire.rs` | `iommu/api/src/acquire.rs` | `ba27f4231592a2f38e3a586cc98cc6069c4ba56e107f23d1f32a91d4d339b497` |
| `programs/src/driver/iommu/api/src/call.rs` | `iommu/api/src/call.rs` | `a337a9c428a99d518e00782ee5b43d7f353ec3ee27abd42eefcdf4df6f818578` |
| `programs/src/driver/iommu/api/src/establish.rs` | `iommu/api/src/establish.rs` | `0dfa71f8bd3b061204948cce92f85eb7c04fe6135a291c778706fc25f302647b` |
| `programs/src/driver/iommu/api/src/fail.rs` | `iommu/api/src/fail.rs` | `00c440fd0ba25b0884d6b77781969257ebdaa4e88de9f66e11144f86f1e5a3d1` |
| `programs/src/driver/iommu/api/src/said.rs` | `iommu/api/src/said.rs` | `ac0da01aa35bdbeb57644a8522f19fa847237959a7580257e872e36086bcc1f3` |
| `programs/src/driver/iommu/api/src/step.rs` | `iommu/api/src/step.rs` | `0bdd10d0da6c2b2eb6c35c28db87756f73ec3fc0c41b1badf4ee8f27f524f6c2` |
| `programs/src/driver/iommu/api/src/wire.rs` | `iommu/api/src/wire.rs` | `a4d2b54a1e7c2e8fec35cbf79669ffef82dd96e42aa1a1a8afe34fd3c70bc56c` |
| `programs/src/driver/iommu/hold/Cargo.toml` | `iommu/hold/Cargo.toml` | `ee8d69541873f8edd4cb5ec47bfb22826de46ee8fab3d94d0e4de083f766077e` |
| `programs/src/driver/iommu/hold/src/lib.rs` | `iommu/hold/src/lib.rs` | `fb759a83403cc130ac60638c245bccbbc4bdcade24e2f93314cb710feaf3c01d` |
| `programs/src/driver/iommu/hold/src/close.rs` | `iommu/hold/src/close.rs` | `75e1c31a2fb7268b42b5e65e28b9b2ab09484a320ee2f35ad262a69af36d9167` |
| `programs/src/driver/iommu/hold/src/effects.rs` | `iommu/hold/src/effects.rs` | `35137da99c6a21b24abc52ef2c6460d0efad4291fdc7dec86b3639892434c0ac` |
| `programs/src/driver/iommu/hold/src/failure.rs` | `iommu/hold/src/failure.rs` | `98232deb9f5d4dfeb5b594a400e2549b8d5b68da7fee9ac3433bfded76f855ee` |
| `programs/src/driver/iommu/hold/src/holds.rs` | `iommu/hold/src/holds.rs` | `6942178bc19ef60f8c07fc317ea7f7a04a376ad937cee35cf16ef32927ba5ef4` |
| `programs/src/driver/iommu/hold/src/memory.rs` | `iommu/hold/src/memory.rs` | `a3448281b8b14ed2257d95d51c69ee5daf91d6eda3f695c89dc41e087442ebe5` |
| `programs/src/driver/iommu/hold/src/phase.rs` | `iommu/hold/src/phase.rs` | `341ba3db1cb6f1afd13c868a4c6e972387d84fda2b2bda674a0f9b5e01f4d0c7` |
| `programs/src/driver/iommu/hold/src/platform.rs` | `iommu/hold/src/platform.rs` | `caafaddea02bc7d1db1923ebac3ccf48ea365bec04366a403a624bc228e95a2d` |
| `programs/src/driver/iommu/hold/src/range.rs` | `iommu/hold/src/range.rs` | `1421824929afee588f0eead45dd5457a6282a51af7725c1101f3efdd9a1d11f9` |
| `programs/src/driver/iommu/hold/src/registration.rs` | `iommu/hold/src/registration.rs` | `09b58f985ec8587cdf34584e676e9f91ced8595237cdbadec601af4d0ccf478c` |
| `programs/src/driver/iommu/hold/src/scope.rs` | `iommu/hold/src/scope.rs` | `c1b78489b8582bbca2d13adf942dc5e4ef508b02468e593ecda949690d69b95e` |

All 26 production files are byte-identical (`26 / 26`). The selected packages
contain no `sqware.*` interface id; none was invented or renamed. No `prog-*`
image target is owned here: Source still declares `prog-iommu`.

## Test ownership

| Source | Provider | Tests | Source SHA-256 | Provider SHA-256 |
| --- | --- | --- | --- | --- |
| `programs/tests/iommu-pages/provider_wire.rs` | `iommu/api/tests/provider_wire.rs` | 4 | `92c2ba7fedaeeaa68d9b0a76c14916beb4391b03cd364afdf143fdb5cf1bcc17` | `fb832fe8a246ad54f46d81add5c50abb6f715a5407894ea4d60517b5edc8eb34` |
| `programs/src/driver/iommu/hold/tests/unit/range.rs` | `iommu/hold/tests/unit/range.rs` | 1 | `22c4bab0c35ef74a0a2759a70a3c4a1137874ae1a0dbc1b08c83cc000d3bb282` | `22c4bab0c35ef74a0a2759a70a3c4a1137874ae1a0dbc1b08c83cc000d3bb282` |
| `programs/tests/retention/ordering.rs` | `iommu/hold/tests/cases/ordering.rs` | 6 | `6ebc0dd671ecb7c2733e419e678a421d2858693a48f8693fab184cea4a988771` | `6ebc0dd671ecb7c2733e419e678a421d2858693a48f8693fab184cea4a988771` |
| `programs/tests/retention/accessor.rs` | `iommu/hold/tests/support/accessor.rs` | fixture | `023a0ad1ea15f922bd9a37fe836e1985da7115d4fcb6fa7596eee4fb9d8d31cd` | `023a0ad1ea15f922bd9a37fe836e1985da7115d4fcb6fa7596eee4fb9d8d31cd` |
| `programs/tests/retention/platform.rs` | `iommu/hold/tests/support/platform.rs` | fixture | `ba2084a21b6f8148bfb2ca06ddde174221216fcdc84a4f4cb5b1d872efd06ed2` | `ba2084a21b6f8148bfb2ca06ddde174221216fcdc84a4f4cb5b1d872efd06ed2` |
| `programs/tests/retention/references.rs` | `iommu/hold/tests/support/references.rs` | fixture | `a9b68e19a0d33f8043d3919bbe39b04808c7ec32de48dd2eaafe2bd209476eeb` | `a9b68e19a0d33f8043d3919bbe39b04808c7ec32de48dd2eaafe2bd209476eeb` |

`provider_wire.rs` changes only the harness imports: `env::{PAGE_SIZE,
PieToken, TaskId}`, `use iommu_api as api`, and `crate::PAGE_SIZE` becomes the
public `env::PAGE_SIZE`. No assertion or test body changed, and no duplicate
`PAGE_SIZE` constant was added. The ordering case file and all three support
fixtures are byte-identical; the new `iommu/hold/tests/ordering.rs` root only
supplies the test-crate module paths and `iommu_hold as retention`.

The three fixtures are used only by `ordering.rs`; when the later Source
consumer removes `ordering.rs`, `accessor.rs`, `platform.rs`, `references.rs`,
and their four module declarations, no other retention case is dropped.

Test mapping and counts:

| Source case | Provider owner | Count |
| --- | --- | --- |
| `programs/tests/iommu-pages/provider_wire.rs` | `iommu/api/tests/provider_wire.rs` | 4 unchanged bodies |
| `programs/src/driver/iommu/hold/tests/unit/range.rs` | `iommu-hold` library unit test | 1 unchanged |
| `programs/tests/retention/ordering.rs` | `iommu/hold/tests/cases/ordering.rs` | 6 unchanged |
| — | `iommu/api/tests/golden.rs` | 7 added public-API golden/boundary cases |

IOMMU-owned host tests: **18** = 4 legacy API + 7 added API + 1 range + 6
ordering. Both `--locked` and `--locked --offline` runs pass 11/11 API and 7/7
hold cases. Source baseline before any Source edit was 52 iommu-pages tests
(including the 4 provider-wire cases) and 45 retention tests (including the 6
ordering cases); after the later consumer removal those suites should be 48
and 39 unless later work changes them. This snapshot does not edit or delete
those Source tests.

The added golden file covers exact byte strings and boundaries for all six
operations, `Establish` 13 / `Acquire` 61 / `Step` 42 / `Said` 38 in a 48-byte
buffer / `Wire` 64, truncated/trailing/unknown-opcode rejection, full-width
64-bit opaque Domain key and Hub id, zero and `112` opaque keys, all seven
fail codes plus the `Bad` fallback, entry/back/party marks, and `Call::back`
token selection for every variant. It uses only public API and does not
install a Kernel, call runtime syscalls, or fake a Kernel host.

## New provider-only files

| File | SHA-256 |
| --- | --- |
| `iommu/api/tests/golden.rs` | `15cd56ec8f3263c40b7ca00683922788fce8dc143bdb9678cab7109bec6b89bf` |
| `iommu/hold/tests/ordering.rs` | `53a14dc8e4ffb8348cef65fbcdfb50c53590a8985f3e62402599276a485f161f` |
| `iommu/Cargo.toml` | `262f83c3cf2f4ecce3d2bcede5e6ba85c3c6739c9ddba566402a03e7fa25bffc` |
| `iommu/.cargo/config.toml` | `ee7170f505ce87a5a75fcf34ef3ba340b3dd32589c8b8b4554b6a641c51efcb5` |
| `iommu/.gitignore` | `95af8a5680ca44ce2f331799f908cb2d2f3dc86efcb57846def37e721f48dde7` |
| `iommu/rust-toolchain.toml` | `3d6cdf686f7dc362f1ce8af733e72339e3cb4ba8996dc9e31aef4682998e0fb6` |
| `iommu/AGENTS.md` | `b5ab92c47ad574e500d2f67757e4b27f9dfe9c1fd9f4a403e8f35e0d6a6ed548` |

## Lockfile and verification

`Cargo.lock` was generated by Cargo with `cargo generate-lockfile --offline`
from the public pins above and then exercised with `--locked` and
`--locked --offline`. It resolves 15 packages: the three local packages and the
pinned/registry closure. No patch, replace, local Git backlink, vendoring, or
hand-written lock entry is present.

Verification artifacts are outside this repository at
`/home/anreonyr/Develop/shuyra/evidence/pr22/provider/`, including command
lines, working directories, exit codes, logs, metadata audits, the old-to-new
hash map, and the final manifest. At worker handoff no commit, push, publication,
or public full SHA existed. Root publishes after independent review; this
snapshot does not establish Kernel independence or Stage 3/4 completion.
