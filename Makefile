CC = clang
CFLAGS = -Wall -Werror -I./include -I./lib/libtelnet-0.23
# LDFLAGS = -L./lib
# LDLIBS = -ltelnet-0.23

SRCS = $(wildcard src/*.c)
OBJS = $(SRCS:.c=.o)
LIB_SRCS = $(wildcard lib/libtelnet-0.23/*.c)
LIB_OBJS = $(LIB_SRCS:.c=.o)

.PHONY: all clean

all: bin/telzine

bin/telzine: $(OBJS) $(LIB_OBJS)
	$(CC) $(LDFLAGS) $^ $(LDLIBS) -o $@

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

lib/libtelnet-0.23/%.o: lib/libtelnet-0.23/%.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJS) $(LIB_OBJS) bin/telzine