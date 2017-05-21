
PKGDIR=$(CARGO_MANIFEST_DIR)

BUILDDIR=$(OUT_DIR)
OSSLDIR=$(PKGDIR)/openssl
SRCDIR=$(PKGDIR)/src

VPATH=$(BUILDDIR):$(SRCDIR):$(OSSLDIR)

.PHONY: debug release default

default:
	cargo check

debug:
	BUILD_FLAGS="-O0 -g -Wall" make libosl.a

release:
	BUILD_FLAGS="-O2" make libosl.a

libosl.a: libssl.a
	gcc -c -fPIC $(BUILD_FLAGS) -o $(BUILDDIR)/osl.o $(SRCDIR)/ssl.c -I$(OSSLDIR)/include/openssl -L$(OSSLDIR) -lssl -lcrypto
	ar rcs $(BUILDDIR)/libosl.a $(BUILDDIR)/osl.o

libssl.a: config
	cd $(OSSLDIR); ./config no-ssl2 no-ssl3 no-shared enable-ec_nistp_64_gcc_128 no-err no-srp
	make -C $(OSSLDIR)

config:
	git submodule init
	git submodule update
