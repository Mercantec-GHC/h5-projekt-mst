
MAKEFLAGS += -j16

CXX=g++
LD=g++

CXXFLAGS = -std=c++23 -pedantic-errors -Wall -Wextra -Wconversion
LDFLAGS =

libraries = libmosquitto

CXXFLAGS += $(shell pkgconf $(libraries) --cflags --exists)
LDFLAGS += $(shell pkgconf $(libraries) --libs --exists)

RELEASE = 0
GDB = 0

ifeq ($(RELEASE),1)
	CXXFLAGS += -O3
else
	CXXFLAGS += -g -Wno-unused

	ifeq ($(GDB),1)
		CXXFLAGS += -ggdb
	endif
endif

ASAN = 0
ifeq ($(ASAN),1)
	CXXFLAGS += -fsanitize=address
endif

build_dir = build
obj_dir = $(build_dir)/obj
src_dir = src
test_dir = tests

sources_without_main = $(shell find $(src_dir) -name '*.cpp' -and -not -name 'main.cpp')
main_source = src/main.cpp
sources = $(sources_without_main) $(main_source)

objects_without_main = $(sources_without_main:%.cpp=$(obj_dir)/%.o)
main_object = $(main_source:%.cpp=$(obj_dir)/%.o)
objects = $(objects_without_main) $(main_object)

target = $(build_dir)/backend

test_sources = $(shell find $(test_dir) -name '*.cpp')
test_objects = $(test_sources:$(test_dir)/%.cpp=$(obj_dir)/$(test_dir)/%.o)
test_targets = $(test_sources:$(test_dir)/%.cpp=$(build_dir)/$(test_dir)/test_%)

all: $(target) $(test_targets)

$(target): $(objects)
	$(LD) -o $@ $(CXXFLAGS) $^ $(LDFLAGS)

$(obj_dir)/%.o:%.cpp
	@mkdir -p $(dir $@)
	$(CXX) $< -c -o $@ -MMD -MP $(CXXFLAGS)

test: $(test_targets)
	printf "%s\n" $^ | xargs -I % sh -c 'echo "- %..." && ./% && echo "- %: OK" || (echo "- %: FAILED" && exit 1)'

$(build_dir)/$(test_dir)/test_%: $(test_objects) $(objects_without_main)
	@mkdir -p $(dir $@)
	$(LD) -o $@ $(CXXFLAGS) $^ $(LDFLAGS)

format:
	clang-format -style=file -i src/*

check:
	clang-format --dry-run --Werror -style=file src/*

clean:
	rm -rf $(build_dir)

-include $(sources:%.cpp=$(obj_dir)/%.d) $(tests:%.cpp=$(obj_dir)/%.d)

