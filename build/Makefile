LIBCLANG_PATH=/usr/lib
DYLIB_EXT=.so

ifeq ($(OS),Windows_NT)
    # Not sure where default location is on windows
	DYLIB_EXT=.dll
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Darwin)
        LIBCLANG_PATH=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib
		DYLIB_EXT=.dylib
    endif
endif

all:
	rm -rf $(OUT_DIR)/*
	ln -sf $(LIBCLANG_PATH)/libclang$(DYLIB_EXT) $(OUT_DIR)

