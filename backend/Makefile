
MAKEFLAGS += -j16

CC=gcc
LD=gcc

CFLAGS=-std=c23 -pedantic-errors -Wall -Wextra -Wconversion
LDFLAGS=

CFLAGS+=-g

# CFLAGS+=-ggdb
CFLAGS+=-fsanitize=address

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

clean:
	rm -rf $(build_dir)

-include $(sources:%.c=$(obj_dir)/%.d)

