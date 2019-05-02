$(shell mkdir -p $(HOME)/.local/run)
export XDG_RUNTIME_DIR ?= $(HOME)/.local/run
export LIBGL_ALWAYS_SOFTWARE ?= true
export LIBGL_ALWAYS_INDIRECT ?= 1
export WAYLAND_DEBUG ?= 1
RUN = $(shell if test -z "$(DBUS_SESSION_BUS_ADDRESS)" && which dbus-launch > /dev/null; then echo dbus-launch; else echo env; fi)

run: build/server
	$(RUN) build/server

build/server: build/build.ninja *.cpp *.h
	ninja -C build

build/build.ninja:
	meson build
