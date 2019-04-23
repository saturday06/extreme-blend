export XDG_RUNTIME_DIR ?= /tmp
export LIBGL_ALWAYS_SOFTWARE ?= true

run: build/server
	build/server

build/server: build/build.ninja *.cpp *.h
	ninja -C build

build/build.ninja:
	meson build
