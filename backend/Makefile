
MAKEFLAGS += -j16

CC=gcc
LD=gcc

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

build_dir = build
obj_dir = $(build_dir)/obj

sources = \
	src/main.c

target=$(build_dir)/backend

all: $(target)

check:
	clang-format --dry-run --Werror -style=file:.clang-format.github src/*

$(target): $(sources:%.c=$(obj_dir)/%.o)
	$(LD) -o $@ $(CFLAGS) $(LDFLAGS) $^

$(obj_dir)/%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $< -c -o $@ -MMD -MP $(CFLAGS)

clean:
	rm -rf $(build_dir)

-include $(sources:%.c=$(obj_dir)/%.d)

