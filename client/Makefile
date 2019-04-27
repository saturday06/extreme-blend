$(shell mkdir -p $(HOME)/.local/run)
export XDG_RUNTIME_DIR ?= $(HOME)/.local/run
export LIBGL_ALWAYS_SOFTWARE ?= true
export WAYLAND_DEBUG ?= 1

run: build/client
	./build/client

build/client: build/build.ninja *.cpp
	ninja -C build

build/build.ninja:
	meson build
