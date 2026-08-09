# Copr SCM build integration.
#
# Configure the copr package with:
#   Clone URL                      = https://github.com/<you>/amdgpu_top
#   Subdirectory                   = (leave empty)
#   Spec file                      = rpm/amdgpu_top.spec
#   Build method / SRPM build type = make srpm
#
# Copr then runs, in a chroot that *does* have network access:
#   make -f <repo>/.copr/Makefile srpm outdir=<dir> spec=<spec>
#
# Note that copr sets the working directory to the package's configured
# subdirectory, which is not necessarily the repository root, so every path
# below is anchored to this Makefile's own location rather than to $(CURDIR).
#
# The resulting SRPM carries the vendored crates, so the actual RPM build runs
# fully offline in every target chroot.

# <repo>/.copr/Makefile -> <repo>
srcroot := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))..)

# Both are normally supplied by copr.
spec   ?= $(srcroot)/rpm/amdgpu_top.spec
outdir ?= $(CURDIR)/srpm

.PHONY: srpm installdeps

installdeps:
	dnf -y install rpm-build cargo git tar zstd

srpm: installdeps
	set -eu; \
	version=$$(rpm --query --queryformat '%{VERSION}\n' --specfile $(spec) | head -n1); \
	name=$$(rpm --query --queryformat '%{NAME}\n' --specfile $(spec) | head -n1); \
	prefix="$$name-$$version"; \
	sources=$$(mktemp -d -p "$${TMPDIR:-/var/tmp}"); \
	trap 'rm -rf "$$sources"' EXIT; \
	git config --global --add safe.directory $(srcroot); \
	git -C $(srcroot) archive --format=tar.gz --prefix="$$prefix/" \
	    -o "$$sources/$$prefix.tar.gz" HEAD; \
	$(srcroot)/rpm/make-vendor-tarball.sh "$$sources/$$prefix.tar.gz" "$$sources"; \
	snapshot=""; \
	if ! git -C $(srcroot) describe --exact-match --tags HEAD 2>/dev/null \
	        | grep -qx "v$$version"; then \
	    snapshot=$$(date -u +%Y%m%d)git$$(git -C $(srcroot) rev-parse --short HEAD); \
	fi; \
	mkdir -p $(outdir); \
	rpmbuild -bs $(spec) \
	    --define "_sourcedir $$sources" \
	    --define "_srcrpmdir $(outdir)" \
	    $${snapshot:+--define "gitsnapshot $$snapshot"}
