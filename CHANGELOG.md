# Changelog

All notable changes to this project will be documented in this file. See [Conventional Commits](https://conventionalcommits.org) for commit guidelines.

## [7.3.0-5](https://github.com/teqm/libvmod-unleash/compare/v7.3.0-4...v7.3.0-5) (2024-07-15)

### Bug Fixes

- `sort_by` doesn't return ([50a16c9](https://github.com/teqm/libvmod-unleash/commit/50a16c98a69d266babd1ca91378c8188e61a6e20))

### Features

- add .resolve_all() method ([#46](https://github.com/teqm/libvmod-unleash/issues/46)) ([a5f0e17](https://github.com/teqm/libvmod-unleash/commit/a5f0e17c086a13462f0000fa0748682f8820b67a))

### Miscellaneous Tasks

- update `unleash-client` ([#44](https://github.com/teqm/libvmod-unleash/issues/44)) ([ce123ee](https://github.com/teqm/libvmod-unleash/commit/ce123eee5f3173bcdedbab21f5924fd2c39f385e))

## [7.3.0-4](https://github.com/teqm/libvmod-unleash/compare/v7.3.0-3...v7.3.0-4) (2024-02-29)

### Bug Fixes

- sort enabled toggles ([#40](https://github.com/teqm/libvmod-unleash/issues/40)) ([4e1e5b4](https://github.com/teqm/libvmod-unleash/commit/4e1e5b41f1e6f1827a6b3079f3fb44d3e339b2ab))

## [7.3.0-3](https://github.com/teqm/libvmod-unleash/compare/v7.3.0-2...v7.3.0-3) (2023-10-27)

### Miscellaneous Tasks

- bump `unleash-client` ([#35](https://github.com/teqm/libvmod-unleash/issues/35)) ([eed4362](https://github.com/teqm/libvmod-unleash/commit/eed4362df59d23fe3c62f339d19628c595076897))

## [7.3.0-2](https://github.com/teqm/libvmod-unleash/compare/v7.3.0-1...v7.3.0-2) (2023-06-01)

### Bug Fixes

- stop setting global tracing subscriber ([#29](https://github.com/teqm/libvmod-unleash/issues/29)) ([21bfb52](https://github.com/teqm/libvmod-unleash/commit/21bfb5280e004bad2d060d7f07f1df2b4d08a097))

### Documentation

- fix vcl syntax ([570d105](https://github.com/teqm/libvmod-unleash/commit/570d105be7c69a60bac08c9477b1d39c916bd03c))

### Miscellaneous Tasks

- use published version of `unleash-client` crate ([#21](https://github.com/teqm/libvmod-unleash/issues/21)) ([710332b](https://github.com/teqm/libvmod-unleash/commit/710332b9b07f49ea281372621c8851425ecd0bf9))
- bump `unleash-client` ([#30](https://github.com/teqm/libvmod-unleash/issues/30)) ([a45e245](https://github.com/teqm/libvmod-unleash/commit/a45e245410bc2754f4dfeea4f18ac56755546142))

## [7.3.0-1](https://github.com/teqm/libvmod-unleash/compare/v6.0.11-1...v7.3.0-1) (2023-05-11)

### Documentation

- fix cliff config ([#19](https://github.com/teqm/libvmod-unleash/issues/19)) ([dc777e3](https://github.com/teqm/libvmod-unleash/commit/dc777e3c380b9f1ae900c054838020b7a14ef695))

### Features

- use varnish 7.3 ([#18](https://github.com/teqm/libvmod-unleash/issues/18)) ([543f4b5](https://github.com/teqm/libvmod-unleash/commit/543f4b51e93cb2a424c90ef63497b0da6e7d1dae))

## [6.0.11-1](https://github.com/teqm/libvmod-unleash/compare/v0.1.0...v6.0.11-1) (2023-05-10)

### Continuous Integration

- remove unnecessary step ([f9575aa](https://github.com/teqm/libvmod-unleash/commit/f9575aa709c0b990f3cab184c5d023a88c4e2eab))
- remove codename from deb package ([14acfc7](https://github.com/teqm/libvmod-unleash/commit/14acfc7ec8869bfe7c8815846358c3ca4b12e931))
- lock unleash server version ([#15](https://github.com/teqm/libvmod-unleash/issues/15)) ([03539aa](https://github.com/teqm/libvmod-unleash/commit/03539aae9efc149e27be98c5463a9b8436bd69dc))

### Documentation

- fix vcl examples ([e9fb23a](https://github.com/teqm/libvmod-unleash/commit/e9fb23a75bcc1ae2f8880bf3111cbbbe9a6f0725))

### Features

- add pkg revision ([#16](https://github.com/teqm/libvmod-unleash/issues/16)) ([281ff20](https://github.com/teqm/libvmod-unleash/commit/281ff20f0d2524e7c9330baed93059d37c09d035))

## 0.1.0 (2023-05-09)

### Continuous Integration

- add ci workflow ([79d8893](https://github.com/teqm/libvmod-unleash/commit/79d88931fbdbdb66dda20ac977aab272c035ab48))
- reorder steps ([b71447f](https://github.com/teqm/libvmod-unleash/commit/b71447f6b91b43cd2972610e2dece602bcbd9b1e))
- load image into docker use ([44fbcb8](https://github.com/teqm/libvmod-unleash/commit/44fbcb8aacc54d03c47a1fb5a5c4154e064abe63))
- fix `VMOD_VERSION` env step ([59efd95](https://github.com/teqm/libvmod-unleash/commit/59efd95e0d17757810021015ff470a10fc318b82))
- use `github.ref_name` ([fdc72f3](https://github.com/teqm/libvmod-unleash/commit/fdc72f3e833dc76794f687efd866684134a441e4))
- fix permissions ([35fdd67](https://github.com/teqm/libvmod-unleash/commit/35fdd674a3569ae9233717c05d27db127e940d59))
- fix github release step ([afe7eb0](https://github.com/teqm/libvmod-unleash/commit/afe7eb0ce69de38db7edd3e7054b058f38450c00))
- add release pr workflow ([149e268](https://github.com/teqm/libvmod-unleash/commit/149e268b9ad0bbcb7e2fd1da58e1ffe0caaf4155))
- fix permissions ([9d69560](https://github.com/teqm/libvmod-unleash/commit/9d69560cd6bb6477f130f40b1433c21149fc0436))
- fix pr changelog body ([c62fe83](https://github.com/teqm/libvmod-unleash/commit/c62fe832b54431001eb5ba336e8ed8dbbc6527ab))
- cleanup git-cliff output ([664c44d](https://github.com/teqm/libvmod-unleash/commit/664c44d8ccb894eb291ccbef2246586941eed17f))
- change pr body contents ([ebea157](https://github.com/teqm/libvmod-unleash/commit/ebea1575de19fb57f96398f4676325cda28a6b75))
- remove `run-check` job ([#9](https://github.com/teqm/libvmod-unleash/issues/9)) ([8d7a0f4](https://github.com/teqm/libvmod-unleash/commit/8d7a0f4859f5335c878434e1a6c28c8f09e189f5))
- add arm64 support for debian package ([#14](https://github.com/teqm/libvmod-unleash/issues/14)) ([d290ec5](https://github.com/teqm/libvmod-unleash/commit/d290ec54609945b566065158822bd438613bd11c))
- add codename to deb package ([3ae6e05](https://github.com/teqm/libvmod-unleash/commit/3ae6e05ab1f710d72fe0e47be2ca72d3ba4ccc7a))
- fix matrix ([2a295e7](https://github.com/teqm/libvmod-unleash/commit/2a295e7fbff7a6636c116cc2b520b1819b7fa959))

### Documentation

- init readme ([b765e6a](https://github.com/teqm/libvmod-unleash/commit/b765e6adad064de1415330207f38a93fb0dbb291))
- improve readme ([c93a7a6](https://github.com/teqm/libvmod-unleash/commit/c93a7a6b3b15316ff2a58bfda6c09cfcfee6600d))
- vcl syntax in examples ([8e12734](https://github.com/teqm/libvmod-unleash/commit/8e127344830109fa9ab495a85641a8fcd46b6ee5))
- update `vmod.vcc` ([#13](https://github.com/teqm/libvmod-unleash/issues/13)) ([8d2888d](https://github.com/teqm/libvmod-unleash/commit/8d2888de5f41d227f27af4535d6bd0384f7acda4))

### Features

- init `vmod_unleash` ([63228c0](https://github.com/teqm/libvmod-unleash/commit/63228c069c4808b83a770196a32bff8e09edb81d))
- build debian package ([#1](https://github.com/teqm/libvmod-unleash/issues/1)) ([0c75a8c](https://github.com/teqm/libvmod-unleash/commit/0c75a8cddfce475f2d33770516bde43b92e2d1a9))
- add rpm support ([#2](https://github.com/teqm/libvmod-unleash/issues/2)) ([24b2730](https://github.com/teqm/libvmod-unleash/commit/24b2730ba1ac24f8da77ec5439bc87e0fb33e55d))
- add jwt support ([#10](https://github.com/teqm/libvmod-unleash/issues/10)) ([43e0328](https://github.com/teqm/libvmod-unleash/commit/43e0328c09c2518baf131b8fa539c8437e078f6e))
- add `disable_metrics` option ([#12](https://github.com/teqm/libvmod-unleash/issues/12)) ([c190039](https://github.com/teqm/libvmod-unleash/commit/c1900395e8ade8c7499f0bb05db0f4682732d855))

### Miscellaneous Tasks

- add git-cliff config ([726d932](https://github.com/teqm/libvmod-unleash/commit/726d932671bd0f7807434754a54bb09aad053dfc))
- release v0.1.0 ([fb012b6](https://github.com/teqm/libvmod-unleash/commit/fb012b6caf0dafce6d9c77351b55e9349cc487ee))
- update changelog ([35e639a](https://github.com/teqm/libvmod-unleash/commit/35e639a9b110af1b4bdd3b8e5cc0843b78f46eab))
- update `unleash-client` ([#3](https://github.com/teqm/libvmod-unleash/issues/3)) ([f1a5e2f](https://github.com/teqm/libvmod-unleash/commit/f1a5e2fa32ea67983d04d0c69dd5b8d4e8cacf34))
- reset changelog ([09e7de3](https://github.com/teqm/libvmod-unleash/commit/09e7de343c10a935795d4382ec765c4090d0700e))
- add LICENSE ([ff5bc94](https://github.com/teqm/libvmod-unleash/commit/ff5bc94afda5bb330a0e9eb2bd5519141db3b292))

### Tests

- add `.is_enabled` smoke test ([fd4ff1d](https://github.com/teqm/libvmod-unleash/commit/fd4ff1d29c7850a5d313fe78612e0b25ce1db294))
- add e2e tests ([45f92f9](https://github.com/teqm/libvmod-unleash/commit/45f92f9158690b0d9e300853d2f3c52f2c7628ac))
- add small delay to e2e tests ([56d7150](https://github.com/teqm/libvmod-unleash/commit/56d715088a7a8f0c9073fa2f43fd69f9c3fc7363))
- add `.get_hash` tests ([ad4da2a](https://github.com/teqm/libvmod-unleash/commit/ad4da2a049aa5900fd47fcdd0830cde133d34108))
- update unleash seed data ([#8](https://github.com/teqm/libvmod-unleash/issues/8)) ([4dbd7cd](https://github.com/teqm/libvmod-unleash/commit/4dbd7cdfed6ee0605de618e4920ec0edd064630c))
- add `try_props_from_str` test ([#11](https://github.com/teqm/libvmod-unleash/issues/11)) ([d95b0de](https://github.com/teqm/libvmod-unleash/commit/d95b0de4d4b2176b28f8e18e0c6728232b8100c9))

<!-- generated by git-cliff -->
