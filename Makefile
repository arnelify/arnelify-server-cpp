# Boot Makefile
# See https://www.gnu.org/software/make/manual/make.html for more about make.
#
# Add the following to your IncludePath settings:
# C/C++ IntelliSense Plugin > Settings > Clang_format_fallback Style > Google
#	"includePath": [
#		"${workspaceFolder}/src/",
#		"${workspaceFolder}/src/cpp"
#		"/opt/homebrew/include",
#		"/opt/homebrew/include/json",
#	]

# ENGINE
ENGINE_BUILD = g++
ENGINE_WATCH = clang++
ENGINE_FLAGS = -std=c++2b

# PATH
PATH_BIN = ./build/index.so
PATH_SRC = ./src/cpp/ffi.cpp
PATH_TESTS_BIN = ./src/tests/bin/index
PATH_TESTS_SRC = ./src/tests/index.cpp

# INC
INC_SRC = -I ./src
INC_CPP = -I ./src/cpp
INC_INCLUDE = -L /usr/include
INC_JSONCPP = -I /usr/include/jsoncpp/json

INC = ${INC_SRC} ${INC_CPP} ${INC_INCLUDE} ${INC_JSONCPP}

# LINK
LINK_JSONCPP = -ljsoncpp
LINK_ZLIB = -lz
LINK = ${LINK_JSONCPP} ${LINK_ZLIB}

# SCRIPTS
build:
	clear && mkdir -p ./build && rm -rf ./build/*
	${ENGINE_BUILD} ${ENGINE_FLAGS} ${INC} ${LINK} -fPIC -shared ${PATH_SRC} -o ${PATH_BIN}

tests:
	clear && mkdir -p ./src/tests/bin && rm -rf ./src/tests/bin/*
	${ENGINE_WATCH} $(ENGINE_FLAGS) $(PATH_TESTS_SRC) ${INC} ${LINK} -o $(PATH_TESTS_BIN) && $(PATH_TESTS_BIN)

.PHONY: build tests