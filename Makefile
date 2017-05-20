
BUILDDIR=$(OUT_DIR)
OSSLDIR="openssl"
SRCDIR="src"

default:
	cargo check

debug:
	BUILD_FLAGS="-O0 -g -Wall" ; make osl

release:
	BUILD_FLAGS="-O2" ; make osl

osl: $(OSSLDIR)/libssl.a $(OSSLDIR)/libcrypto.a
	gcc -c -fPIC $(BUILD_FLAGS) -o $(BUILDDIR)/osl.o $(SRCDIR)/ssl.c -I$(OSSLDIR)/include/openssl -L$(BUILDDIR) -lssl -lcrypto
	ar rcs $(BUILDDIR)/libosl.a $(BUILDDIR)/osl.o

libssl.a: $(OSSLDIR)/config
	cd $(OSSLDIR); ./config no-ssl2 no-ssl3 no-shared enable-ec_nistp_64_gcc_128 no-err no-srp
	make -C $(OSSLDIR)
	cp $(OSSLDIR)/$@ $(BUILDDIR)

libcrypto.a: libssl.a
	cp $(OSSLDIR)/$@ $(BUILDDIR)

$(OSSLDIR)/config:
	git submodule init
	git submodule update

wiefj_build:
	gcc -c -fPIC -O0 -g -Wall -o osl.o src/ssl.c -I../include/openssl -L../ -lssl -lcrypto -lm -lpthread
	ar rcs target/debug/deps/libosl.a osl.o
	rm -f target/debug/tmp
	cargo build --verbose

wqoi_release:
	gcc -c -fPIC -O2 -Wall -o osl.o src/ssl.c -I../include/openssl -L../ -lssl -lcrypto -lm -lpthread
	ar rcs target/release/deps/libosl.a osl.o
	rm -f target/release/tmp
	cargo build --release

run:
	cargo run
