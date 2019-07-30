#if 1
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
#include <sys/stat.h>
#include <unistd.h>
#include <wayland-client-protocol.h>
#include <wayland-client.h>
#include <xdg-shell-client-header.h>

////////////////////////////////////////////////////////////// header
///////////////////////////////////////////
/* wayland 繝��繧ｿ */

typedef struct {
  struct wl_display *display;
  struct wl_registry *registry;
  struct wl_compositor *compositor;
  struct wl_shm *shm;
  struct xdg_wm_base *xdg_wm_base;

  uint32_t shm_cnt;
} wayland;

/* 繧ｦ繧｣繝ｳ繝峨え繧､繝｡繝ｼ繧ｸ繝��繧ｿ */

typedef struct {
  struct wl_shm_pool *pool;
  struct wl_buffer *buffer;
  void *data;
  int width, height, size;
} imagebuf;

/*---- func ----*/

wayland *wayland_init(void);
void wayland_finish(wayland *p);

imagebuf *imagebuf_create(wayland *wl, int width, int height);
void imagebuf_destroy(imagebuf *p);

/////////////////////////////////////////////////////// sub
///////////////////////////////////////////

//========================
// imagebuf
//========================

/** POSIX 蜈ｱ譛峨Γ繝｢繝ｪ繧ｪ繝悶ず繧ｧ繧ｯ繝医ｒ菴懈� */

static int _create_posix_shm(wayland *p) {
  char name[64];
  int ret;

  while (1) {
    snprintf(name, 64, "/wayland-test-%x", p->shm_cnt);

    ret = shm_open(name, O_CREAT | O_EXCL | O_RDWR | O_CLOEXEC, 0600);

    if (ret >= 0) {
      shm_unlink(name);
      p->shm_cnt++;

      break;
    } else if (errno == EEXIST)
      //蜷悟錐縺悟ｭ伜惠縺吶ｋ
      p->shm_cnt++;
    else if (errno != EINTR)
      break;
  }

  return ret;
}

/** wl_shm_pool 菴懈� */

static struct wl_shm_pool *_create_shm_pool(wayland *p, int size,
                                            void **ppbuf) {
  int fd;
  void *data;
  struct wl_shm_pool *pool;

  fd = _create_posix_shm(p);
  if (fd < 0)
    return NULL;

  //繧ｵ繧､繧ｺ螟画峩

  if (ftruncate(fd, size) < 0) {
    close(fd);
    return NULL;
  }

  //繝｡繝｢繝ｪ縺ｫ繝槭ャ繝斐Φ繧ｰ

  data = mmap(NULL, size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);

  if (data == MAP_FAILED) {
    close(fd);
    return NULL;
  }

  // wl_shm_pool 菴懈�

  pool = wl_shm_create_pool(p->shm, fd, size);

  close(fd);

  *ppbuf = data;

  return pool;
}

//========================
// xdg_wm_base
//========================

static void _xdg_wm_base_ping(void *data, struct xdg_wm_base *xdg_wm_base,
                              uint32_t serial) {
  xdg_wm_base_pong(xdg_wm_base, serial);
}

static const struct xdg_wm_base_listener g_xdg_wm_base_listener = {
    _xdg_wm_base_ping};

//========================
// wl_registry
//========================

static void _registry_global(void *data, struct wl_registry *reg, uint32_t id,
                             const char *itf, uint32_t ver) {
  wayland *p = (wayland *)data;

  if (strcmp(itf, "wl_compositor") == 0)
    p->compositor = static_cast<wl_compositor *>(
        wl_registry_bind(reg, id, &wl_compositor_interface, 1));
  else if (strcmp(itf, "wl_shm") == 0)
    p->shm =
        static_cast<wl_shm *>(wl_registry_bind(reg, id, &wl_shm_interface, 1));
  else if (strcmp(itf, "xdg_wm_base") == 0) {
    p->xdg_wm_base = static_cast<xdg_wm_base *>(
        wl_registry_bind(reg, id, &xdg_wm_base_interface, 1));

    xdg_wm_base_add_listener(p->xdg_wm_base, &g_xdg_wm_base_listener, NULL);
  }
}

static void _registry_global_remove(void *data, struct wl_registry *registry,
                                    uint32_t id) {}

static const struct wl_registry_listener g_registry_listener = {
    _registry_global, _registry_global_remove};

//========================
//
//========================

/** Wayland 邨ゆｺ� */

void wayland_finish(wayland *p) {
  if (p) {
    if (p->xdg_wm_base) {
      xdg_wm_base_destroy(p->xdg_wm_base);
    }
    if (p->shm) {
      wl_shm_destroy(p->shm);
    }
    if (p->compositor) {
      wl_compositor_destroy(p->compositor);
    }
    if (p->registry) {
      wl_registry_destroy(p->registry);
    }
    if (p->display) {
      wl_display_disconnect(p->display);
    }

    free(p);
  }
}

/** Wayland 蛻晄悄蛹� */

wayland *wayland_init(void) {
  wayland *p;
  struct wl_display *disp;

  p = (wayland *)calloc(1, sizeof(wayland));
  if (!p)
    return NULL;

  //謗･邯�

  disp = p->display = wl_display_connect(NULL);

  if (!disp) {
    printf("failed wl_display_connect()\n");
    exit(1);
  }

  // wl_registry

  p->registry = wl_display_get_registry(disp);

  wl_registry_add_listener(p->registry, &g_registry_listener, p);

  // wl_registry 縺ｮ繧､繝吶Φ繝亥�逅�′邨ゅｏ繧九∪縺ｧ蠕�▽

  wl_display_roundtrip(disp);

  // xdg_wm_base 縺後↑縺�

  if (!p->xdg_wm_base) {
    printf("not find 'xdg_wm_base'\n");
    wayland_finish(p);
    exit(1);
  }

  return p;
}

//------- imagebuf

/** 繧､繝｡繝ｼ繧ｸ繝舌ャ繝輔ぃ菴懈� */

imagebuf *imagebuf_create(wayland *wl, int width, int height) {
  imagebuf *img;
  struct wl_shm_pool *pool;
  struct wl_buffer *buffer;
  void *data;
  int size;

  size = width * 4 * height;

  // wl_shm_pool 菴懈�

  pool = _create_shm_pool(wl, size, &data);
  if (!pool)
    return NULL;

  // wl_buffer 菴懈�

  buffer = wl_shm_pool_create_buffer(pool, 0, width, height, width * 4,
                                     WL_SHM_FORMAT_XRGB8888);

  if (!buffer)
    goto ERR;

  // imagebuf 菴懈�

  img = (imagebuf *)malloc(sizeof(imagebuf));
  if (!img)
    goto ERR;

  img->pool = pool;
  img->buffer = buffer;
  img->data = data;
  img->width = width;
  img->height = height;
  img->size = size;

  return img;

ERR:
  wl_shm_pool_destroy(pool);
  munmap(data, size);
  return NULL;
}

/** imagebuf 蜑企勁 */

void imagebuf_destroy(imagebuf *p) {
  if (p) {
    wl_buffer_destroy(p->buffer);
    wl_shm_pool_destroy(p->pool);
    munmap(p->data, p->size);

    free(p);
  }
}

////////////////////////////////////////////////////////////////////////////////

//-----------------

wayland *g_wl;

/* ウィンドウ */
typedef struct {
  struct wl_surface *surface;
  struct xdg_surface *xdg_surface;
  struct xdg_toplevel *xdg_toplevel;
  imagebuf *img;
  int configure_flag, close_flag;
} window;

void window_update(window *p);

//-----------------

/* イメージ描画 */

static void _draw_image(imagebuf *p) {
  uint8_t *pd = (uint8_t *)p->data;
  int i;

  for (i = p->width * p->height; i > 0; i--, pd += 4) {
    pd[0] = 0;
    pd[1] = 0;
    pd[2] = 255;
    pd[3] = 0;
  }
}

//-------- xdg_surface

static void _xdg_surface_configure(void *data, struct xdg_surface *surface,
                                   uint32_t serial) {
  window *win = (window *)data;

  printf("surface-configure: serial %u\n", serial);

  xdg_surface_ack_configure(surface, serial);

  if (win->configure_flag == 0) {
    win->configure_flag = 1;

    _draw_image(win->img);
    window_update(win);
  }
}

static const struct xdg_surface_listener g_xdg_surface_listener = {
    _xdg_surface_configure};

//-------- xdg_toplevel

static void _xdg_toplevel_configure(void *data, struct xdg_toplevel *toplevel,
                                    int32_t width, int32_t height,
                                    struct wl_array *states) {
  void *vps;

  printf("toplevel-configure: w %d, h %d / states: ", width, height);

  wl_array_for_each(vps, states) {
    switch (*static_cast<uint32_t *>(vps)) {
    case XDG_TOPLEVEL_STATE_MAXIMIZED:
      printf("MAXIMIZED ");
      break;
    case XDG_TOPLEVEL_STATE_FULLSCREEN:
      printf("FULLSCREEN ");
      break;
    case XDG_TOPLEVEL_STATE_RESIZING:
      printf("RESIZING ");
      break;
    case XDG_TOPLEVEL_STATE_ACTIVATED:
      printf("ACTIVATED ");
      break;
    }
  }

  printf("\n");
}

static void _xdg_toplevel_close(void *data, struct xdg_toplevel *toplevel) {
  printf("close\n");

  ((window *)data)->close_flag = 1;
}

const struct xdg_toplevel_listener g_xdg_toplevel_listener = {
    _xdg_toplevel_configure, _xdg_toplevel_close};

//--------- window

/* ウィンドウ作成 */

window *window_create(wayland *wl, int width, int height) {
  window *p;

  p = (window *)calloc(1, sizeof(window));
  if (!p)
    return NULL;

  // wl_surface

  p->surface = wl_compositor_create_surface(wl->compositor);

  // xdg_surface

  p->xdg_surface = xdg_wm_base_get_xdg_surface(wl->xdg_wm_base, p->surface);

  xdg_surface_add_listener(p->xdg_surface, &g_xdg_surface_listener, p);

  // xdg_toplevel

  p->xdg_toplevel = xdg_surface_get_toplevel(p->xdg_surface);

  xdg_toplevel_add_listener(p->xdg_toplevel, &g_xdg_toplevel_listener, p);

  //適用

  wl_surface_commit(p->surface);

  //イメージ作成

  p->img = imagebuf_create(wl, width, height);

  return p;
}

/* ウィンドウ破棄 */

void window_destroy(window *p) {
  if (p) {
    imagebuf_destroy(p->img);

    xdg_toplevel_destroy(p->xdg_toplevel);
    xdg_surface_destroy(p->xdg_surface);
    wl_surface_destroy(p->surface);

    free(p);
  }
}

/* ウィンドウ更新 */

void window_update(window *p) {
  wl_surface_attach(p->surface, p->img->buffer, 0, 0);
  wl_surface_damage(p->surface, 0, 0, p->img->width, p->img->height);
  wl_surface_commit(p->surface);
}

//--------------

TEST(client_test, dev06) {
  window *win;

  g_wl = wayland_init();

  win = window_create(g_wl, 256, 256);

  //

  while (wl_display_dispatch(g_wl->display) != -1 && win->close_flag == 0)
    ;

  //

  window_destroy(win);

  wayland_finish(g_wl);
}

#endif
