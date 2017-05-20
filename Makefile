
PKGDIR=$(CARGO_MANIFEST_DIR)

BUILDDIR=$(OUT_DIR)
OSSLDIR="$(PKGDIR)/openssl"
SRCDIR="$(PKGDIR)/src"

default:
	cargo check

debug:
	BUILD_FLAGS="-O0 -g -Wall" ; make osl

release:
	BUILD_FLAGS="-O2" ; make osl

osl: $(BUILDDIR)/libssl.a $(BUILDDIR)/libcrypto.a
	gcc -c -fPIC $(BUILD_FLAGS) -o $(BUILDDIR)/osl.o $(SRCDIR)/ssl.c -I$(OSSLDIR)/include/openssl -L$(BUILDDIR) -lssl -lcrypto
	ar rcs $(BUILDDIR)/libosl.a $(BUILDDIR)/osl.o

$(BUILDDIR)/libssl.a: $(OSSLDIR)/config
	cd $(OSSLDIR); ./config no-ssl2 no-ssl3 no-shared enable-ec_nistp_64_gcc_128 no-err no-srp
	make -C $(OSSLDIR)
	mv $(OSSLDIR)/libssl.a $(BUILDDIR)

$(BUILDDIR)/libcrypto.a: $(BUILDDIR)/libssl.a
	mv $(OSSLDIR)/libcrypto.a $(BUILDDIR)

$(OSSLDIR)/config:
	git submodule init
	git submodule update