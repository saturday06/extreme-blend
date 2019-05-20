#include <errno.h>
#include <extreme-blend/reflector.h>
#include <fcntl.h>
#include <glog/logging.h>
#include <gtest/gtest.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <unistd.h>
#include <wayland-client.h>

using namespace ExtremeBlend;

class CompositorFixture : public ::testing::Test {
protected:
  virtual void SetUp() { reflector.reset(new Reflector()); };

  virtual void TearDown() { reflector.reset(); };

  std::unique_ptr<Reflector> reflector;
};

// TEST_F(CompositorFixture, foo) {
//    struct wl_display *display;
//    display = wl_display_connect(NULL);
//    if (!display) {
//        ASSERT_TRUE(false);
//    }
//    wl_display_disconnect(display);
//}

struct wl_display *display = NULL;
struct wl_compositor *compositor = NULL;
struct wl_surface *surface = NULL;
struct wl_shell *shell = NULL;
struct wl_shell_surface *shell_surface = NULL;
struct wl_shm *shm = NULL;

struct wl_buffer *buffer = NULL;
void *shm_data = NULL;
int WIDTH = 640, HEIGHT = 480;

static void shm_format(void *data, struct wl_shm *wl_shm, uint32_t format) {
  fprintf(stderr, "Format: %d\n", format);
}

static wl_shm_listener shm_listener = {shm_format};

static void global_registry_handler(void *data, struct wl_registry *registry,
                                    uint32_t id, const char *interface,
                                    uint32_t version) {
  LOG(INFO) << "global_registry_handler " << interface << " " << id;
  if (strcmp(interface, "wl_compositor") == 0) {
    compositor = (struct wl_compositor *)wl_registry_bind(
        registry, id, &wl_compositor_interface, 1);
  } else if (strcmp(interface, "wl_shell") == 0) {
    shell = (struct wl_shell *)wl_registry_bind(registry, id,
                                                &wl_shell_interface, 1);
  } else if (strcmp(interface, "wl_shm") == 0) {
    shm = (struct wl_shm *)wl_registry_bind(registry, id, &wl_shm_interface, 1);
    wl_shm_add_listener(shm, &shm_listener, NULL);
  }
}

static void global_registry_remover(void *data, struct wl_registry *registry,
                                    uint32_t id) {
  printf("Got a registry losing event for %d\n", id);
}

static const struct wl_registry_listener registry_listener = {
    global_registry_handler, global_registry_remover};

static void handle_ping(void *data, struct wl_shell_surface *shell_surface,
                        uint32_t serial) {
  wl_shell_surface_pong(shell_surface, serial);
}

static void handle_configure(void *data, struct wl_shell_surface *shell_surface,
                             uint32_t edges, int32_t width, int32_t height) {}

static void handle_popup_done(void *data,
                              struct wl_shell_surface *shell_surface) {}

static const struct wl_shell_surface_listener shell_surface_listener = {
    handle_ping, handle_configure, handle_popup_done};

int os_create_anonymous_file(int size) {
  static const char fmt[] = "/shared-XXXXXX";
  const char *path = NULL;
  char *name = NULL;
  int fd = -1;
  path = getenv("XDG_RUNTIME_DIR");
  if (!path) {
    errno = ENOENT;
    return -1;
  }
  name = (char *)malloc(strlen(path) + sizeof(fmt));
  if (!name) {
    return -1;
  }
  strcpy(name, path);
  strcat(name, fmt);

  fd = mkostemp(name, O_CLOEXEC);
  if (fd >= 0) {
    unlink(name);
  }

  free(name);
  name = NULL;
  if (fd < 0) {
    return -1;
  }
  if (ftruncate(fd, size) < 0) {
    close(fd);
    return -1;
  }
  return fd;
}

void create_window() {
  struct wl_shm_pool *pool;
  int stride = WIDTH * 4;
  int size = stride * HEIGHT;
  int fd = -1;
  buffer = 0;

  /* create shared memory */
  fd = os_create_anonymous_file(size);
  if (fd < 0) {
    fprintf(stderr, "creating a buffer file for %d Bytes failed:", size);
    exit(1);
  }
  shm_data = mmap(NULL, size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
  if (shm_data == NULL) {
    fprintf(stderr, "mmap failed");
    close(fd);
    exit(1);
  }
  pool = wl_shm_create_pool(shm, fd, size);
  buffer = wl_shm_pool_create_buffer(pool, 0, WIDTH, HEIGHT, stride,
                                     WL_SHM_FORMAT_XRGB8888);
  wl_shm_pool_destroy(pool);

  wl_surface_attach(surface, buffer, 0, 0);
  wl_surface_damage(surface, 0, 0, WIDTH, HEIGHT);
  wl_surface_commit(surface);
}

TEST_F(CompositorFixture, bar) {
  display = wl_display_connect(NULL);
  ASSERT_TRUE(display != NULL);

  struct wl_registry *registry = wl_display_get_registry(display);
  wl_registry_add_listener(registry, &registry_listener, NULL);

  wl_display_dispatch(display);
  wl_display_roundtrip(display);

  ASSERT_TRUE(compositor != NULL);
  ASSERT_TRUE(shell != NULL);
  ASSERT_TRUE(shm != NULL);

  surface = wl_compositor_create_surface(compositor);
  ASSERT_TRUE(surface != NULL);

  shell_surface = wl_shell_get_shell_surface(shell, surface);
  ASSERT_TRUE(shell_surface != NULL);

  wl_shell_surface_set_toplevel(shell_surface);
  wl_shell_surface_add_listener(shell_surface, &shell_surface_listener, NULL);

  create_window();

  wl_shell_surface_set_title(shell_surface, "sample");

  while (wl_display_dispatch(display) != -1) {
    ;
  }

  wl_display_disconnect(display);
}
