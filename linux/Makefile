run: build/server
	build/server

build/server: build/build.ninja *.cpp *.h
	ninja -C build

build/build.ninja:
	meson build
