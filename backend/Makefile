
MAKEFLAGS += -j16

CXX=g++
LD=g++
CLANG_FORMAT=clang-format

CXXFLAGS = -std=c++23 -pedantic-errors -Wall -Wextra -Wconversion
LDFLAGS =

# libraries = ""

# CXXFLAGS += $(shell pkg-config $(libraries) --cflags)
# LDFLAGS += $(shell pkg-config $(libraries) --libs)

RELEASE = 0
GDB = 0

ifeq ($(RELEASE),1)
	CXXFLAGS += -O3
else
	CXXFLAGS += -g -Wno-unused

	ifeq ($(GDB),1)
		CXXFLAGS += -ggdb
	else
		CXXFLAGS += -fsanitize=address
	endif
endif

NO_MAIN = 0
ifeq ($(NO_MAIN),1)
	CXXFLAGS += -Dmain="not_main"
endif

build_dir = build
obj_dir = $(build_dir)/obj

sources = \
	src/main.cpp \
	src/tcp.cpp \
	src/http.cpp


target=$(build_dir)/backend

all: $(target)

$(target): $(sources:%.cpp=$(obj_dir)/%.o)
	$(LD) -o $@ $(CXXFLAGS) $(LDFLAGS) $^

$(obj_dir)/%.o: %.cpp
	@mkdir -p $(dir $@)
	$(CXX) $< -c -o $@ -MMD -MP $(CXXFLAGS)

format:
	$(CLANG_FORMAT) -style=file -i src/*

check:
	$(CLANG_FORMAT) --dry-run --Werror -style=file src/*

clean:
	rm -rf $(build_dir)

-include $(sources:%.cpp=$(obj_dir)/%.d)

