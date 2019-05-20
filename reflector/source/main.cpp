#include <cassert>
#include <cstdio>
#include <cstdlib>
#include <error.h>
#include <memory>
#include <wayland-server.h>

#include "compositor.h"
#include "zxdg_shell_v6.h"
#include <extreme-blend/reflector.h>
#include <gflags/gflags.h>
#include <glog/logging.h>

bool egl_init(wl_display *wl_disp);

int main2(int argc, char *argv[]);

#ifndef CUSTOM_MAIN

int main(int argc, char *argv[]) { return main2(argc, argv); }

#endif

int main2(int argc, char *argv[]) {
  if (setenv("WAYLAND_DEBUG", "1", 1)) {
    perror(argv[0]);
    return 1;
  }

  if (setenv("XDG_RUNTIME_DIR", "/tmp", 1)) {
    perror(argv[0]);
    return 1;
  }

  google::InitGoogleLogging(argv[0]);
  FLAGS_logtostderr = true;
  // google::ParseCommandLineFlags(&argc, &argv, true);
  LOG(INFO) << "start";
  ExtremeBlend::Reflector r;
  r.wait_for_exit();
  return 0;
  /*
      wl_display *display = wl_display_create();

      if (!egl_init(display)) {
          fprintf(stderr, "No EGL support\n");
      }

      const char *socket_name = wl_display_add_socket_auto(display);
      assert(socket_name);

      auto compositor = std::make_unique<Compositor>(display);
      auto shell = std::make_unique<ZxdgShellV6>(display);

      LOG(INFO) << "hello, wayland";
      wl_display_run(display); */
}
