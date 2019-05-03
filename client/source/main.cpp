#include <stdio.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/mman.h>

#include <wayland-client.h>

int main2(void);

#ifndef ULTIMATE_DESKTOP_CLIENT_CUSTOM_MAIN

int main(void) {
    return main2();
}

#endif

#if 0

int main2(void) {
    struct wl_display *display;

    display = wl_display_connect(NULL);
    if (!display) {
        printf("can not connect\n");
        return 1;
    }

    printf("connect\n");

    wl_display_disconnect(display);

    printf("disconnect\n");

    return 0;
}
#endif

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

static wl_shm_listener shm_listener = {
        shm_format
};


static void global_registry_handler(
        void *data,
        struct wl_registry *registry,
        uint32_t id,
        const char *interface,
        uint32_t version) {
    printf("global_registry_handler %s %d\n", interface, id);
    if (strcmp(interface, "wl_compositor") == 0) {
        compositor = (struct wl_compositor *) wl_registry_bind(registry, id, &wl_compositor_interface, 1);
    } else if (strcmp(interface, "wl_shell") == 0) {
        shell = (struct wl_shell *) wl_registry_bind(registry, id, &wl_shell_interface, 1);
    } else if (strcmp(interface, "wl_shm") == 0) {
        shm = (struct wl_shm *) wl_registry_bind(registry, id, &wl_shm_interface, 1);
        wl_shm_add_listener(shm, &shm_listener, NULL);
    }
}

static void global_registry_remover(void *data, struct wl_registry *registry, uint32_t id) {
    printf("Got a registry losing event for %d\n", id);
}


static const struct wl_registry_listener registry_listener = {
        global_registry_handler,
        global_registry_remover
};


static void handle_ping(
        void *data,
        struct wl_shell_surface *shell_surface,
        uint32_t serial) {
    wl_shell_surface_pong(shell_surface, serial);
}

static void handle_configure(
        void *data,
        struct wl_shell_surface *shell_surface,
        uint32_t edges,
        int32_t width,
        int32_t height) {
}

static void handle_popup_done(void *data, struct wl_shell_surface *shell_surface) {
}

static const struct wl_shell_surface_listener shell_surface_listener = {
        handle_ping,
        handle_configure,
        handle_popup_done
};

int os_create_anonymous_file(int size) {
    static const char fmt[] = "/weston-shared-XXXXXX";
    const char *path = NULL;
    char *name = NULL;
    int fd = -1;
    path = getenv("XDG_RUNTIME_DIR");
    if (!path) {
        errno = ENOENT;
        return -1;
    }
    name = (char *) malloc(strlen(path) + sizeof(fmt));
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
    buffer = wl_shm_pool_create_buffer(pool, 0, WIDTH, HEIGHT, stride, WL_SHM_FORMAT_XRGB8888);
    wl_shm_pool_destroy(pool);

    wl_surface_attach(surface, buffer, 0, 0);
    wl_surface_damage(surface, 0, 0, WIDTH, HEIGHT);
    wl_surface_commit(surface);
}

int main2(void) {
    display = wl_display_connect(NULL);
    if (display == NULL) {
        fprintf(stderr, "Can't connect to display\n");
        exit(1);
    }
    printf("connected to display\n");

    struct wl_registry *registry = wl_display_get_registry(display);
    wl_registry_add_listener(registry, &registry_listener, NULL);

    wl_display_dispatch(display);
    wl_display_roundtrip(display);

    if (compositor == NULL) {
        printf("Can't find compositor\n");
        return 1;
    }

    if (shell == NULL) {
        printf("Can't find shell\n");
        return 1;
    }

    if (shm == NULL) {
        printf("Can't find shm\n");
        return 1;
    }

    surface = wl_compositor_create_surface(compositor);
    if (surface == NULL) {
        fprintf(stderr, "Can't create surface\n");
        return 1;
    }

    shell_surface = wl_shell_get_shell_surface(shell, surface);
    if (shell_surface == NULL) {
        printf("Can't create shell surface\n");
        return 1;
    }
    wl_shell_surface_set_toplevel(shell_surface);
    wl_shell_surface_add_listener(shell_surface, &shell_surface_listener, NULL);

    create_window();

    wl_shell_surface_set_title(shell_surface, "sample");

    while (wl_display_dispatch(display) != -1) { ;
    }

    wl_display_disconnect(display);
    printf("disconnected from display\n");
    return 0;
}
