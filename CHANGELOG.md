# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.1.0](https://github.com/compilersEllie/keeps_and_kobolds/compare/23d9f80d9dc43c75eec490295a5be1303bbe8ff7..0.1.0) - 2026-09-05
#### Features
- (**preferences**) Create a preferences struct with load/save - ([43f4d49](https://github.com/compilersEllie/keeps_and_kobolds/commit/43f4d49060e39804cd40c1f853d201f6a8a0eb36)) - Ellie Pratt
- (**stats**) Ensure data struct and enum fields handle default values 'properly' [conveniently] - ([a9f1916](https://github.com/compilersEllie/keeps_and_kobolds/commit/a9f1916c45af81c234c6235588e971c82a19577a)) - Ellie Pratt
- (**stats**) Add support for Height and Age (values and limits) - ([0dc5196](https://github.com/compilersEllie/keeps_and_kobolds/commit/0dc519619a4f09d72834dfe733d1d95fcd8576df)) - Ellie Pratt
- (**ui**) Add bind config support for Mouse+KeyModifiers and fix lints - ([9af9a4e](https://github.com/compilersEllie/keeps_and_kobolds/commit/9af9a4ec6ba9a221928ff30655a41a31cf615e9b)) - Ellie Pratt
- Setup initial gossipsub behavior - ([95fc1e9](https://github.com/compilersEllie/keeps_and_kobolds/commit/95fc1e9389f1636af58dcd1e8a9f179a1e408853)) - Ellie Pratt
- Setup initial kad+mdns behavior - ([4079532](https://github.com/compilersEllie/keeps_and_kobolds/commit/40795320fd7125d6982755026e7ada7986f03a3b)) - Ellie Pratt
- Implement demo of basic ping single hop p2p networking - ([5c025a2](https://github.com/compilersEllie/keeps_and_kobolds/commit/5c025a2e5896fc6fbbdbe716d66d694c6f2eac0c)) - Ellie Pratt
- Stub out App setup/teardown - ([e75f2ef](https://github.com/compilersEllie/keeps_and_kobolds/commit/e75f2ef1be39fd18a98f2b2923a738b0c57807fa)) - Ellie Pratt
- Add Relationship, Pos, Vec2D structs, movement effects and Key/mouse binding defaults - ([4dd0052](https://github.com/compilersEllie/keeps_and_kobolds/commit/4dd00529925c0522354bb90ecd4e5e70f39f0f08)) - Ellie Pratt
- Create an initial data model for the Character, Stats, Background etx. - ([2322247](https://github.com/compilersEllie/keeps_and_kobolds/commit/232224750ce7bf2bd36838c647e3624aa6e6e600)) - Ellie Pratt
#### Bug Fixes
- typo Cobold -> Kobold - ([ca192da](https://github.com/compilersEllie/keeps_and_kobolds/commit/ca192dae0dede3807a639effecb7202900b39eab)) - Ellie Pratt
#### Documentation
- Add README - ([5b574e2](https://github.com/compilersEllie/keeps_and_kobolds/commit/5b574e2b0155d74d188696e39b89a0e95e666fa6)) - Ellie Pratt
#### Miscellaneous Chores
- (**clippy**) Silence warnings that should be future work - ([b9ee5c8](https://github.com/compilersEllie/keeps_and_kobolds/commit/b9ee5c843014c4aacdc4beb8c03743177193b93b)) - Ellie Pratt
- (**version**) 0.1.0 - ([1f840bc](https://github.com/compilersEllie/keeps_and_kobolds/commit/1f840bce2ca6bd319aaea4407e9e1928837d1a6c)) - Ellie Pratt
- Use 'cargo workspaces' to version and publish with cog - ([31f7c84](https://github.com/compilersEllie/keeps_and_kobolds/commit/31f7c8483ec94669ecccb0fb348d9e9e886be540)) - Ellie Pratt
- Set github actions to not upload while storage is full - ([1c8307b](https://github.com/compilersEllie/keeps_and_kobolds/commit/1c8307ba793fb920533d2ac2db4b94c493c8d6e2)) - Ellie Pratt
- Update github actions workflow for rust - ([4d57a5f](https://github.com/compilersEllie/keeps_and_kobolds/commit/4d57a5f7e342073ae4168f1322cce7d00b5ac081)) - Ellie Pratt
- Update github username - ([38d9efa](https://github.com/compilersEllie/keeps_and_kobolds/commit/38d9efa3fdd9439016ce03f56fe85098e0799fcd)) - Ellie Pratt
- cargo fmt - ([19dd14b](https://github.com/compilersEllie/keeps_and_kobolds/commit/19dd14b9b4ddb85d31c8790fe4e529fb1b9f99d0)) - Ellie Pratt
- Add a test map using png perlin noise - ([a1df346](https://github.com/compilersEllie/keeps_and_kobolds/commit/a1df34685f27b269d6932f108a94a3c6bb1b15c9)) - Ellie Pratt
- Add cog config and plan ci/release work - ([bae8b48](https://github.com/compilersEllie/keeps_and_kobolds/commit/bae8b48d23f48953ae36b2dadaef065de1d74073)) - Ellie Pratt
- Add license file - ([c289219](https://github.com/compilersEllie/keeps_and_kobolds/commit/c2892193d9c99bc945ee01b35d188bfe9ae96786)) - Ellie Pratt
- Plan feature work - ([6c95c34](https://github.com/compilersEllie/keeps_and_kobolds/commit/6c95c346476ca32ef1e5f18e2f6030012ac94577)) - Ellie Pratt
- Clean up typo file for 'olf' ancestry - ([f20e78b](https://github.com/compilersEllie/keeps_and_kobolds/commit/f20e78ba05d4769642fa97a1f4e965dfa77c8284)) - Ellie Pratt
- Fix generated Ancestry names - ([59b8bc9](https://github.com/compilersEllie/keeps_and_kobolds/commit/59b8bc92e44665be22c74e676e2d628e73ea21d0)) - Ellie Pratt
- Move readme - ([128a78a](https://github.com/compilersEllie/keeps_and_kobolds/commit/128a78a803d830070d50ac8a6262ade3547ed84a)) - Ellie Pratt
- Rename mob to character - ([06d2e62](https://github.com/compilersEllie/keeps_and_kobolds/commit/06d2e6262fadec234ddc2c2abb325722c01cc0e7)) - Ellie Pratt
- Track unfinished modules - ([de37e9d](https://github.com/compilersEllie/keeps_and_kobolds/commit/de37e9df3e3e8e7f9d9e3d0f298c38e031680d64)) - Ellie Pratt
- Declare modules - ([0e2a4ac](https://github.com/compilersEllie/keeps_and_kobolds/commit/0e2a4acfc3d8495540cd95b6cf8a7b87076821ae)) - Ellie Pratt
- Add github workflow for rust - ([c28b7ad](https://github.com/compilersEllie/keeps_and_kobolds/commit/c28b7ad7c7afa4128e7d64f9e9721bd9a1a4779d)) - Ellie Pratt
- Init - ([23d9f80](https://github.com/compilersEllie/keeps_and_kobolds/commit/23d9f80d9dc43c75eec490295a5be1303bbe8ff7)) - Ellie Pratt

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).