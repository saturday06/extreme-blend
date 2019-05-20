#pragma once

#include <wayland-server.h>

class Toplevel {
public:
  Toplevel(wl_client *client, uint32_t id);

  static void destroy(wl_client *client, wl_resource *resource);

  static void set_parent(wl_client *client, wl_resource *resource,
                         wl_resource *parent);

  static void set_title(wl_client *client, wl_resource *resource,
                        const char *title);

  static void set_app_id(wl_client *client, wl_resource *resource,
                         const char *app_id);

  static void show_window_menu(wl_client *client, wl_resource *resource,
                               wl_resource *seat, uint32_t serial, int32_t x,
                               int32_t y);

  static void move(wl_client *client, wl_resource *resource, wl_resource *seat,
                   uint32_t serial);

  static void resize(wl_client *client, wl_resource *resource,
                     wl_resource *seat, uint32_t serial, uint32_t edges);

  static void set_max_size(wl_client *client, wl_resource *resource,
                           int32_t width, int32_t height);

  static void set_min_size(wl_client *client, wl_resource *resource,
                           int32_t width, int32_t height);

  static void set_maximized(wl_client *client, wl_resource *resource);

  static void unset_maximized(wl_client *client, wl_resource *resource);

  static void set_fullscreen(wl_client *client, wl_resource *resource,
                             wl_resource *output);

  static void set_minimized(wl_client *client, wl_resource *resource);
};
