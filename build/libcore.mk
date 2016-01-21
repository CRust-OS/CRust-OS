# Patch and recompile libcore to remove float support

LIBCORE_TMP = $(TMP)/libcore

$(LIBCORE_TMP)/rustc-nightly.tar.gz:
	$(MKDIR) $(@D)
	$(WGET) https://static.rust-lang.org/dist/rustc-nightly-src.tar.gz -O $@

$(LIBCORE_TMP)/libcore_nofp.patch:
	$(WGET) https://raw.githubusercontent.com/thepowersgang/rust-barebones-kernel/master/libcore_nofp.patch -O $@

$(LIBCORE_TMP)/rustc-nightly: $(LIBCORE_TMP)/rustc-nightly.tar.gz
	tar xzf $< -C $(@D)
	@touch --no-create $@

$(LIBCORE_TMP)/.libcore_patched_successfully: $(LIBCORE_TMP)/rustc-nightly $(LIBCORE_TMP)/libcore_nofp.patch
	git apply $(LIBCORE_TMP)/libcore_nofp.patch --directory $(LIBCORE_TMP)/rustc-nightly/src/libcore
	@touch $@

$(LIB)/libcore.rlib: $(LIBCORE_TMP)/.libcore_patched_successfully
	$(MKDIR) $(@D)
	rustc --target x86_64-unknown-xen --cfg disable_float -Z no-landing-pads --out-dir $(@D) $(LIBCORE_TMP)/rustc-nightly/src/libcore/lib.rs
