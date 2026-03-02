# Boot Makefile
# See https://www.gnu.org/software/make/manual/make.html for more about make.

# ENGINE
ENGINE_BUILD = g++
ENGINE_WATCH = clang++
ENGINE_FLAGS = -std=c++2b

# PATH
PATH_TEST_BIN = $(CURDIR)/build/main
PATH_TEST_SRC_HTTP1 = $(CURDIR)/tests/tcp1/http1.cpp
PATH_TEST_SRC_HTTP2 = $(CURDIR)/tests/tcp1/http2.cpp
PATH_TEST_SRC_WS = $(CURDIR)/tests/tcp1/ws.cpp
PATH_TEST_SRC_HTTP3 = $(CURDIR)/tests/tcp2/http3.cpp
PATH_TEST_SRC_WT = $(CURDIR)/tests/tcp2/wt.cpp

# INC
INC_CPP = -I $(CURDIR)/src
INC_INCLUDE = -L /usr/include
INC_JSONCPP = -I /usr/include/jsoncpp/json

INC = ${INC_CPP} ${INC_INCLUDE} ${INC_JSONCPP}

# LINK
LINK_NATIVE = -Lnative/target/release -larnelify_server -Wl,-rpath,native/target/release
LINK_JSONCPP = -ljsoncpp
LINK = ${LINK_NATIVE} ${LINK_JSONCPP}

build:
	clear && cd native \
	&& cargo build --release \
	&& cd ..

test_http1:
	clear && mkdir -p build && rm -rf build/*
	${ENGINE_WATCH} $(ENGINE_FLAGS) ${INC} $(PATH_TEST_SRC_HTTP1) ${LINK} -o $(PATH_TEST_BIN) && $(PATH_TEST_BIN)

test_http2:
	clear && mkdir -p build && rm -rf build/*
	${ENGINE_WATCH} $(ENGINE_FLAGS) ${INC} $(PATH_TEST_SRC_HTTP2) ${LINK} -o $(PATH_TEST_BIN) && $(PATH_TEST_BIN)
	
test_ws:
	clear && mkdir -p build && rm -rf build/*
	${ENGINE_WATCH} $(ENGINE_FLAGS) ${INC} $(PATH_TEST_SRC_WS) ${LINK} -o $(PATH_TEST_BIN) && $(PATH_TEST_BIN)

test_http3:
	clear && mkdir -p build && rm -rf build/*
	${ENGINE_WATCH} $(ENGINE_FLAGS) ${INC} $(PATH_TEST_SRC_HTTP3) ${LINK} -o $(PATH_TEST_BIN) && $(PATH_TEST_BIN)

test_wt:
	clear && mkdir -p build && rm -rf build/*
	${ENGINE_WATCH} $(ENGINE_FLAGS) ${INC} $(PATH_TEST_SRC_WT) ${LINK} -o $(PATH_TEST_BIN) && $(PATH_TEST_BIN)

.PHONY: \
	build \
	test_http1 \
	test_http2 \
	test_ws \
	test_http3 \
	test_wt