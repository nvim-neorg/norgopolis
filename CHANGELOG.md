# Changelog

## 0.1.4 (2024-02-21)


### ⚠ BREAKING CHANGES

* convert repo to server-only

### Features

* add `communication.proto` file ([07f799e](https://github.com/nvim-neorg/norgopolis/commit/07f799ef9c07afbb76fd7e6c0d49f7c37e9e493f))
* add `norgopolis_module` crate ([a9a358b](https://github.com/nvim-neorg/norgopolis/commit/a9a358bc283f86a9014d34a8cf00c00812ee5e5e))
* add barebones ([264dfc7](https://github.com/nvim-neorg/norgopolis/commit/264dfc7ca2a55c9566b4e0d74152f67332f78210))
* add basic server barebones ([c7aa300](https://github.com/nvim-neorg/norgopolis/commit/c7aa3008039f885cc37017fd82a39ff88cbbc623))
* add build.rs ([ce36077](https://github.com/nvim-neorg/norgopolis/commit/ce360776e7b1c38cc6fcf0ec34a8726a4d07c66e))
* add clap CLI parser ([025cb42](https://github.com/nvim-neorg/norgopolis/commit/025cb4239082df737a7921e3a4cc34bfdd996004))
* add hello world client ([7c09724](https://github.com/nvim-neorg/norgopolis/commit/7c0972429cb9990341dc8badc43ca85799887593))
* add hello world module ([24ad534](https://github.com/nvim-neorg/norgopolis/commit/24ad534ca01d221ddfc2ebd9db43b1fb97b5df29))
* add LICENSE ([09553ae](https://github.com/nvim-neorg/norgopolis/commit/09553ae16e39fcd03425b689d47291e12563d172))
* add necessary dependencies for each project ([cd483c6](https://github.com/nvim-neorg/norgopolis/commit/cd483c63e9b3129a1b45ba6e71138cbce2f73d34))
* add skeleton (but partially unimplemented) code for messagepack streaming ([730b21e](https://github.com/nvim-neorg/norgopolis/commit/730b21e8177f2b208aaa07fbadd09d21270eef22))
* **client:** expose the `communication` module ([5438d52](https://github.com/nvim-neorg/norgopolis/commit/5438d5228be3efba4261850495f8b8444988e692))
* **hello-world-client:** use new `norgopolis-module` library ([d990ada](https://github.com/nvim-neorg/norgopolis/commit/d990adafbf885505932d0673be976b7f059d2cc1))
* limit module search to just `.local/share/norgopolis/modules` ([9acdd55](https://github.com/nvim-neorg/norgopolis/commit/9acdd55e871fd008191a7676da7c1d39d0841c6d))
* **norgopolis-module:** add `encode` function for `MessagePack` ([a8084aa](https://github.com/nvim-neorg/norgopolis/commit/a8084aaffedde701a0bf8a5f19bcffe988504797))
* **norgopolis-module:** finish initial APIs ([2562e92](https://github.com/nvim-neorg/norgopolis/commit/2562e925272ed91526ed0170c7d9d794182e4964))
* optimize `forward` function to be non-blocking ([0ddb6fa](https://github.com/nvim-neorg/norgopolis/commit/0ddb6fafe57f0649783b301f618d471ac4b715c5))
* print "ready" when the server is ready to accept connections ([fecef6c](https://github.com/nvim-neorg/norgopolis/commit/fecef6c014b6fe5c9320c90ba025c23f5139873a))
* **protos:** add `override` rpc method ([f04213d](https://github.com/nvim-neorg/norgopolis/commit/f04213d67bed931469ebf70713bfb0feda7b7dd8))
* **server:** cache running modules, don't spawn a new one on every request ([4582d7d](https://github.com/nvim-neorg/norgopolis/commit/4582d7d80f41294fe26cf5a8ddc4c3b835edb54c))
* **server:** finish first version of the norgopolis server ([a801e67](https://github.com/nvim-neorg/norgopolis/commit/a801e6705f80d956e3dc5f39e52f1ad68da10080))
* **server:** finish skeleton code, no more rust errors ([f529463](https://github.com/nvim-neorg/norgopolis/commit/f529463482597f5cce6622b0293a2d178a482492))
* strip binaries as much as possible ([ddd654d](https://github.com/nvim-neorg/norgopolis/commit/ddd654d18c7800597c4a05ecf0603cdce6b9c2a3))
* time out the program after 5 minutes of inactivity ([fda798d](https://github.com/nvim-neorg/norgopolis/commit/fda798da742d0291c1ec449106651a65b7c2b7c0))


### Bug Fixes

* broken projects in `examples/` after migration ([15e97df](https://github.com/nvim-neorg/norgopolis/commit/15e97df5349cd6f83b1d56bd34ff30b56b65a9f8))
* drain remaining no-shutdown messages when possible ([c01a434](https://github.com/nvim-neorg/norgopolis/commit/c01a4345b7fa29cae603b69a93d12eea23577d88))
* lifetime errors with the `invoke` function ([6468735](https://github.com/nvim-neorg/norgopolis/commit/6468735aa66d81972f12c687eba1e6452a32815f))
* properly clean up running modules on shutdown to prevent zombie processes ([1f957f7](https://github.com/nvim-neorg/norgopolis/commit/1f957f73d24c08be4e6b7e0f25a2fe5c213602b1))
* use unbounded channel to prevent norgopolis from hanging ([95346e3](https://github.com/nvim-neorg/norgopolis/commit/95346e328dc492449cc6ca7f58c224eaac4908fd))


### Code Refactoring

* convert repo to server-only ([ca3b1a3](https://github.com/nvim-neorg/norgopolis/commit/ca3b1a3f20e3bbcb3c72586e6f913db671a1aacf))


### Continuous Integration

* add release-please ([6d3292b](https://github.com/nvim-neorg/norgopolis/commit/6d3292b4d664c04fdb0014bf06ed58d98019128b))
