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

TEST(client_test, dev01) {
  struct wl_display *display;

  display = wl_display_connect(NULL);
  ASSERT_TRUE(display != NULL);

  wl_display_disconnect(display);
}
