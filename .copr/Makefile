# Copr SCM build integration.
#
# Configure the copr package with:
#   Type       = Custom / SCM
#   Clone URL  = https://github.com/Umio-Yasuno/amdgpu_top
#   Build method / SRPM build type = make srpm
#
# Copr then runs, in a chroot that *does* have network access:
#   make -f .copr/Makefile srpm outdir=<dir> spec=<spec>
#
# The resulting SRPM carries the vendored crates, so the actual RPM build runs
# fully offline in every target chroot.

spec   := rpm/amdgpu_top.spec
outdir ?= $(CURDIR)/srpm

.PHONY: srpm installdeps

installdeps:
	dnf -y install rpm-build cargo git tar zstd

srpm: installdeps
	set -eu; \
	version=$$(rpm --query --queryformat '%{VERSION}\n' --specfile $(spec) | head -n1); \
	name=$$(rpm --query --queryformat '%{NAME}\n' --specfile $(spec) | head -n1); \
	prefix="$$name-$$version"; \
	sources=$$(mktemp -d); \
	trap 'rm -rf "$$sources"' EXIT; \
	git config --global --add safe.directory $(CURDIR); \
	git archive --format=tar.gz --prefix="$$prefix/" -o "$$sources/$$prefix.tar.gz" HEAD; \
	./rpm/make-vendor-tarball.sh "$$sources/$$prefix.tar.gz" "$$sources"; \
	snapshot=""; \
	if ! git describe --exact-match --tags HEAD 2>/dev/null | grep -qx "v$$version"; then \
	    snapshot=$$(date -u +%Y%m%d)git$$(git rev-parse --short HEAD); \
	fi; \
	mkdir -p $(outdir); \
	rpmbuild -bs $(spec) \
	    --define "_sourcedir $$sources" \
	    --define "_srcrpmdir $(outdir)" \
	    $${snapshot:+--define "gitsnapshot $$snapshot"}
