# RPM packaging for Fedora / RHEL (copr)

`amdgpu_top.spec` builds `amdgpu_top` for Fedora, RHEL and their derivatives.

## Why a vendor tarball

`libamdgpu_top` depends on `libdrm_amdgpu_sys` by **git revision**, not by a
crates.io version, so the crates cannot be unbundled into individual Fedora
`rust-*` packages the way `rust2rpm` normally arranges. Instead all dependencies
are vendored into a second source tarball and the build runs `cargo build
--offline`. That is also what makes the build work in mock and copr, where the
build chroot has no network access.

`make-vendor-tarball.sh` produces that tarball. It contains `vendor/` plus the
`.cargo/config.toml` emitted by `cargo vendor`, which redirects both crates.io
and the `libdrm_amdgpu_sys` git source at the vendored copies. `%prep` just
unpacks it over the source tree. Roughly 720 MB uncompressed, ~57 MB as
`.tar.zst`.

Generating it therefore needs about 1.5 GB of free scratch space. The script
works next to its output directory rather than in `/tmp`, which is frequently a
small tmpfs; set `VENDOR_TMPDIR` to move that elsewhere. `ZSTD_LEVEL` (default
15) trades compression time and memory against tarball size.

## Building in copr

Point copr at this repository and let `.copr/Makefile` do the work — copr's SRPM
build step has network access, so it can vendor the crates for you:

    copr-cli create amdgpu_top --chroot fedora-42-x86_64 --chroot epel-10-x86_64
    copr-cli add-package-scm amdgpu_top \
        --name amdgpu_top \
        --clone-url https://github.com/Umio-Yasuno/amdgpu_top \
        --spec rpm/amdgpu_top.spec \
        --type git \
        --method make_srpm
    copr-cli build-package amdgpu_top --name amdgpu_top

Builds made from a commit that is not a `vX.Y.Z` tag get a snapshot release such
as `0.11.5-1.20260809git97daebf`.

Copr runs the Makefile with the working directory set to the package's
configured *subdirectory*, so leaving that empty and setting only the spec file
path is the straightforward setup. `.copr/Makefile` locates the repository from
its own path rather than from `$PWD`, so a subdirectory of `rpm` works too.

## Building by hand

    version=0.11.5
    mkdir -p ~/rpmbuild/SOURCES
    spectool -g -R rpm/amdgpu_top.spec            # or: curl the GitHub archive
    ./rpm/make-vendor-tarball.sh \
        ~/rpmbuild/SOURCES/amdgpu_top-$version.tar.gz ~/rpmbuild/SOURCES
    rpmbuild -bs rpm/amdgpu_top.spec
    mock -r fedora-42-x86_64 ~/rpmbuild/SRPMS/amdgpu_top-$version-1.fc42.src.rpm

Note that `spectool -g` only fetches `Source0`; `Source1` is the vendor tarball
you generate locally.

## Build options

`--without gui` drops the egui/wgpu front end and builds only the TUI, SMI and
JSON modes — useful for headless RHEL hosts. It also drops the OFL-1.1 font from
the package, so the `License` tag becomes plain `MIT`.

    rpmbuild -bs --without gui rpm/amdgpu_top.spec

## Chroot notes

* The Rust code is edition 2024, so **rustc >= 1.85** is required. On
  RHEL/EPEL that comes from `rust-toolset`; check that the toolset in your
  target chroot is new enough before adding it to the copr project (EPEL 9 in
  particular has shipped older toolsets).
* `libdrm_amdgpu_sys` is built with its `link_drm` feature, so the binary links
  `-ldrm -ldrm_amdgpu` and the runtime `libdrm` dependency is picked up
  automatically by RPM's dependency generator. Its `buildtime_bindgen` feature
  stays off, so no clang/bindgen is pulled in.
* The `git_version` feature is disabled — there is no `.git` in the tarball, and
  the release version already identifies the build.
