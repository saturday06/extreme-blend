#include "shell.h"

static struct wl_shell_interface shell_interface = {
        &Shell::get_shell_surface
};

void Shell::get_shell_surface(wl_client *client, wl_resource *resource, uint32_t id, wl_resource *surface) {
}
