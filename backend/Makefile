
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

build_dir = build
obj_dir = $(build_dir)/obj
src_dir = src
test_dir = tests

sources = $(shell find $(src_dir) -name '*.cpp' -and -not -name 'main.cpp')
sources_with_main=$(sources) src/main.cpp

tests = $(shell find $(test_dir) -name '*.cpp')

target=$(build_dir)/backend

all: $(target)

$(target): $(sources_with_main:%.cpp=$(obj_dir)/%.o)
	$(LD) -o $@ $(CXXFLAGS) $(LDFLAGS) $^

$(obj_dir)/%.o:%.cpp
	@mkdir -p $(dir $@)
	$(CXX) $< -c -o $@ -MMD -MP $(CXXFLAGS)

test: $(tests:tests/%.cpp=$(build_dir)/tests/test_%)
	printf "%s\n" $^ | xargs -I % sh -c 'echo "- %..." && ./% && echo "- %: OK" || (echo "- %: FAILED" && exit 1)'

$(build_dir)/tests/test_%: $(obj_dir)/tests/%.o $(sources:%.cpp=$(obj_dir)/%.o)
	@mkdir -p $(dir $@)
	$(LD) -o $@ $(CXXFLAGS) $(LDFLAGS) $^

format:
	$(CLANG_FORMAT) -style=file -i src/*

check:
	$(CLANG_FORMAT) --dry-run --Werror -style=file src/*

clean:
	rm -rf $(build_dir)

-include $(sources_with_main:%.cpp=$(obj_dir)/%.d) $(tests:%.cpp=$(obj_dir)/%.d)

