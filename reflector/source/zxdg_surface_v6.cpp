#include "zxdg_surface_v6.h"
#include "toplevel.h"

#include <cassert>
#include <cstdio>
#include <xdg-shell-unstable-v6-server-header.h>

static struct zxdg_surface_v6_interface shell_surface_interface = {
    &ZxdgSurfaceV6::destroy, &ZxdgSurfaceV6::get_toplevel,
    &ZxdgSurfaceV6::get_popup, &ZxdgSurfaceV6::set_window_geometry,
    &ZxdgSurfaceV6::ack_configure};

static void destroy_shell_surface(wl_resource *resource) {
  auto surface =
      static_cast<ZxdgSurfaceV6 *>(wl_resource_get_user_data(resource));
  delete surface;
}

ZxdgSurfaceV6::ZxdgSurfaceV6(wl_client *client, uint32_t id) {
  wl_resource *resource =
      wl_resource_create(client, &zxdg_surface_v6_interface, 1, id);
  assert(resource);

  wl_resource_set_implementation(resource, &shell_surface_interface, this,
                                 &destroy_shell_surface);
}

void ZxdgSurfaceV6::destroy(wl_client *client, wl_resource *resource) {
  fprintf(stderr, "ZxdgSurfaceV6::destroy not implemented\n");
}

void ZxdgSurfaceV6::get_toplevel(struct wl_client *client,
                                 struct wl_resource *resource, uint32_t id) {
  auto toplevel = new Toplevel(client, id);
  assert(toplevel);
}

void ZxdgSurfaceV6::get_popup(wl_client *client, wl_resource *resource,
                              uint32_t id, wl_resource *parent,
                              wl_resource *positioner) {
  fprintf(stderr, "ZxdgSurfaceV6::get_popup not implemented\n");
}

void ZxdgSurfaceV6::set_window_geometry(wl_client *client,
                                        wl_resource *resource, int32_t x,
                                        int32_t y, int32_t width,
                                        int32_t height) {
  fprintf(stderr, "ZxdgSurfaceV6::set_window_geometry not implemented\n");
}

void ZxdgSurfaceV6::ack_configure(wl_client *client, wl_resource *resource,
                                  uint32_t serial) {
  fprintf(stderr, "ZxdgSurfaceV6::ack_configure not implemented\n");
}
