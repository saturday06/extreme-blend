#include "test.h"
#include <errno.h>
#include <fcntl.h>
#include <glog/logging.h>
#include <gtest/gtest.h>
#include <regex>
#include <spawn.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <string>
#include <sys/mman.h>
#include <unistd.h>
#include <wayland-client-protocol.h>
#include <wayland-client.h>

struct wl_compositor *g_compositor = NULL;

static void _registry_global(void *data, struct wl_registry *registry,
                             uint32_t id, const char *interface,
                             uint32_t version) {
  printf("%s, id %u, ver %u\n", interface, id, version);

  if (strcmp(interface, "wl_compositor") == 0) {
    g_compositor = reinterpret_cast<wl_compositor *>(
        wl_registry_bind(registry, id, &wl_compositor_interface, 1));
  }
}

static void _registry_global_remove(void *data, struct wl_registry *registry,
                                    uint32_t id) {}

static const struct wl_registry_listener g_reg_listener = {
    _registry_global, _registry_global_remove};

TEST(client_test, dev02) {
  struct wl_display *display;
  struct wl_registry *reg;

  display = wl_display_connect(NULL);
  ASSERT_TRUE(display != NULL);

  reg = wl_display_get_registry(display);
  ASSERT_TRUE(reg != NULL);
  wl_registry_add_listener(reg, &g_reg_listener, NULL);

  wl_display_roundtrip(display);
  ASSERT_TRUE(g_compositor != NULL);

  //

  wl_compositor_destroy(g_compositor);
  wl_registry_destroy(reg);

  wl_display_disconnect(display);
}
