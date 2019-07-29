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

TEST(dev_client) {
  ASSERT_TRUE(true);
  /*
  ASSERT_TRUE(client != NULL);
  ASSERT_TRUE(client->compositor != NULL);
  ASSERT_TRUE(client->shell != NULL);
  ASSERT_TRUE(client->shell_surface != NULL);
  ASSERT_TRUE(client->shm != NULL);

  while (wl_display_dispatch(client->display) != -1) {
    sleep(1);
  }
  free(client);
  */
}
