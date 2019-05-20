#pragma once

#include <wayland-server.h>

class Shell {
public:
  Shell(wl_display *display);

  static void get_shell_surface(wl_client *client, wl_resource *resource,
                                uint32_t id, wl_resource *surface);

  static void global_bind(wl_client *client, void *data, uint32_t version,
                          uint32_t id);

private:
  wl_display *display;
};
