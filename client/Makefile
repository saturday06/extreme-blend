$(shell mkdir -p $(HOME)/.local/run)
export XDG_RUNTIME_DIR ?= $(HOME)/.local/run
export LIBGL_ALWAYS_SOFTWARE ?= true
export LIBGL_ALWAYS_INDIRECT ?= true
export GALLIUM_DRIVER ?= llvmpipe
export WAYLAND_DEBUG ?= 1
export EGL_LOG_LEVEL ?= debug

run: build/client
	./build/client

build/client: build/build.ninja *.cpp
	ninja -C build

build/build.ninja:
	meson build
