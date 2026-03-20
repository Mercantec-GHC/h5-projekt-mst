
MAKEFLAGS += -j16

CC=gcc
LD=gcc
CLANG_FORMAT=clang-format

CFLAGS = -std=c23 -pedantic-errors -Wall -Wextra -Wconversion
LDFLAGS =

libraries = libmicrohttpd

CFLAGS += $(shell pkg-config $(libraries) --cflags)
LDFLAGS += $(shell pkg-config $(libraries) --libs)

RELEASE = 0
GDB = 0

ifeq ($(RELEASE),1)
	CFLAGS += -O3
else
	CFLAGS += -g -Wno-unused

	ifeq ($(GDB),1)
		CFLAGS += -ggdb
	else
		CFLAGS += -fsanitize=address
	endif
endif

NO_MAIN = 0
ifeq ($(NO_MAIN),1)
	CFLAGS += -Dmain="not_main"
endif

build_dir = build
obj_dir = $(build_dir)/obj

sources = \
	src/main.c

target=$(build_dir)/backend

all: $(target)

$(target): $(sources:%.c=$(obj_dir)/%.o)
	$(LD) -o $@ $(CFLAGS) $(LDFLAGS) $^

$(obj_dir)/%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $< -c -o $@ -MMD -MP $(CFLAGS)

format:
	$(CLANG_FORMAT) -style=file -i src/*

check:
	$(CLANG_FORMAT) --dry-run --Werror -style=file src/*

clean:
	rm -rf $(build_dir)

-include $(sources:%.c=$(obj_dir)/%.d)

