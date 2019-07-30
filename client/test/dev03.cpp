#if 0
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

struct wl_shm *g_shm;

//------------- wl_shm

static void _shm_format(void *data, struct wl_shm *shm, uint32_t format) {
  if (format == WL_SHM_FORMAT_ARGB8888) {
    printf("%u: ARGB8888\n", format);
  } else if (format == WL_SHM_FORMAT_XRGB8888) {
    printf("%u: XRGB8888\n", format);
  } else {
    printf("0x%08X: %c%c%c%c\n", format, (char)(format >> 24),
           (char)(format >> 16), (char)(format >> 8), (char)format);
  }
}

static const struct wl_shm_listener g_shm_listener = {_shm_format};

//------------- wl_registry

static void _registry_global(void *data, struct wl_registry *registry,
                             uint32_t id, const char *interface,
                             uint32_t version) {
  if (strcmp(interface, "wl_shm") == 0) {
    g_shm = reinterpret_cast<wl_shm *>(
        wl_registry_bind(registry, id, &wl_shm_interface, 1));

    wl_shm_add_listener(g_shm, &g_shm_listener, NULL);
  }
}

static void _registry_global_remove(void *data, struct wl_registry *registry,
                                    uint32_t id) {}

static const struct wl_registry_listener g_reg_listener = {
    _registry_global, _registry_global_remove};

//------------

TEST(client_test, dev03) {
  struct wl_display *display;
  struct wl_registry *reg;

  display = wl_display_connect(NULL);
  ASSERT_TRUE(display != NULL);

  reg = wl_display_get_registry(display);
  ASSERT_TRUE(reg != NULL);
  wl_registry_add_listener(reg, &g_reg_listener, NULL);

  wl_display_roundtrip(display);

  wl_display_roundtrip(display);

  ASSERT_TRUE(g_shm != NULL);
  wl_shm_destroy(g_shm);
  wl_registry_destroy(reg);

  wl_display_disconnect(display);
}

#endif
