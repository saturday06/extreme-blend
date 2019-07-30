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

typedef struct {
  struct wl_display *display;
  struct wl_registry *registry;
  struct wl_compositor *compositor;
  struct wl_shm *shm;
  struct wl_shell *shell;

  uint32_t shm_cnt;
} wayland;

typedef struct {
  struct wl_shm_pool *pool;
  struct wl_buffer *buffer;
  void *data;
  int width, height, size;
} imagebuf;

wayland *wayland_init(void);
void wayland_finish(wayland *p);

imagebuf *imagebuf_create(wayland *wl, int width, int height, int enable_alpha);
void imagebuf_destroy(imagebuf *p);

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
  else if (strcmp(itf, "wl_shell") == 0)
    p->shell = static_cast<wl_shell *>(
        wl_registry_bind(reg, id, &wl_shell_interface, 1));
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
    wl_shm_destroy(p->shm);
    wl_shell_destroy(p->shell);
    wl_compositor_destroy(p->compositor);
    wl_registry_destroy(p->registry);

    wl_display_disconnect(p->display);

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
    printf("can not connect\n");
    exit(1);
  }

  // wl_registry

  p->registry = wl_display_get_registry(disp);

  wl_registry_add_listener(p->registry, &g_registry_listener, p);

  // wl_registry 縺ｮ繧､繝吶Φ繝亥�逅�′邨ゅｏ繧九∪縺ｧ蠕�▽

  wl_display_roundtrip(disp);

  return p;
}

/** 繧､繝｡繝ｼ繧ｸ繝舌ャ繝輔ぃ菴懈� */

imagebuf *imagebuf_create(wayland *wl, int width, int height,
                          int enable_alpha) {
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
                                     (enable_alpha) ? WL_SHM_FORMAT_ARGB8888
                                                    : WL_SHM_FORMAT_XRGB8888);

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

//---------------

wayland *g_wl;

typedef struct {
  struct wl_surface *surface;
  imagebuf *img;
  int count, loop;
} callback_data;

static void _callback_done(void *data, struct wl_callback *callback,
                           uint32_t time_ms);

//---------------

/* イメージ描画 */

static void _draw_image(imagebuf *p, int alpha) {
  uint8_t *pd = (uint8_t *)p->data, val;
  int ix, iy;

  for (iy = p->height; iy > 0; iy--) {
    for (ix = p->width, val = 0; ix > 0; ix--, pd += 4) {
      pd[0] = pd[1] = 0;
      pd[2] = val++;
      pd[3] = alpha;
    }
  }
}

//----------- wl_shell_surface

static void _shell_surface_ping(void *data,
                                struct wl_shell_surface *shell_surface,
                                uint32_t serial) {
  wl_shell_surface_pong(shell_surface, serial);
  printf("ping %u\n", serial);
}

static void _shell_surface_configure(void *data,
                                     struct wl_shell_surface *shell_surface,
                                     uint32_t edges, int32_t width,
                                     int32_t height) {
  //サイズ変更時
}

static void _shell_surface_popup_done(void *data,
                                      struct wl_shell_surface *shell_surface) {
  //ポップアップ終了時
}

static const struct wl_shell_surface_listener g_shell_surface_listener = {
    _shell_surface_ping, _shell_surface_configure, _shell_surface_popup_done};

//----------- wl_buffer

static void _buffer_release(void *data, struct wl_buffer *buffer) {
  //サーバーが buffer へのアクセスを終えた時
}

static const struct wl_buffer_listener g_buffer_listener = {_buffer_release};

//----------- wl_callback

static const struct wl_callback_listener g_callback_listener = {_callback_done};

void _callback_done(void *data, struct wl_callback *callback,
                    uint32_t time_ms) {
  callback_data *p = (callback_data *)data;

  wl_callback_destroy(callback);

  p->count++;

  if (p->count == 256)
    p->loop = 0;
  else {
    //更新

    _draw_image(p->img, p->count);

    wl_surface_attach(p->surface, p->img->buffer, 0, 0);
    wl_surface_damage(p->surface, 0, 0, p->img->width, p->img->height);

    /*
        //不透明範囲設定
        struct wl_region *region;
        region = wl_compositor_create_region(g_wl->compositor);
        wl_region_add(region, 0, 0, p->img->width, p->img->height);
        wl_surface_set_opaque_region(p->surface, region);
        wl_region_destroy(region);
    */

    //新しいコールバック

    callback = wl_surface_frame(p->surface);
    wl_callback_add_listener(callback, &g_callback_listener, data);

    //適用

    wl_surface_commit(p->surface);
  }
}

//-----------

TEST(client_test, dev05) {

  g_wl = NULL;
  struct wl_surface *surface;
  struct wl_shell_surface *shell_surface;
  struct wl_callback *callback;
  imagebuf *img;
  callback_data dat;

  g_wl = wayland_init();

  // surface 作成
  ASSERT_TRUE(g_wl->compositor != NULL);
  surface = wl_compositor_create_surface(g_wl->compositor);
  ASSERT_TRUE(g_wl->shell != NULL);
  shell_surface = wl_shell_get_shell_surface(g_wl->shell, surface);

  wl_shell_surface_set_toplevel(shell_surface);

  wl_shell_surface_add_listener(shell_surface, &g_shell_surface_listener, NULL);

  //イメージ作成

  img = imagebuf_create(g_wl, 256, 256, 1);

  wl_buffer_add_listener(img->buffer, &g_buffer_listener, NULL);

  //アニメ用コールバック

  dat.surface = surface;
  dat.img = img;
  dat.count = 0;
  dat.loop = 1;

  callback = wl_surface_frame(surface);

  wl_callback_add_listener(callback, &g_callback_listener, &dat);

  //最初の画面

  _draw_image(img, 0);

  wl_surface_attach(surface, img->buffer, 0, 0);
  wl_surface_commit(surface);

  //

  while (wl_display_dispatch(g_wl->display) != -1 && dat.loop)
    ;

  //

  wl_shell_surface_destroy(shell_surface);
  wl_surface_destroy(surface);

  imagebuf_destroy(img);

  wayland_finish(g_wl);
}

#endif
