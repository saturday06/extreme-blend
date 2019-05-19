#include "shell.h"
#include <assert.h>

static struct wl_shell_interface shell_interface = {
        &Shell::get_shell_surface
};

Shell::Shell(wl_display* display_)
        : display(display_) {
    wl_global_create(display, &wl_shell_interface, 1, this,
                     &Shell::global_bind);
}

void Shell::global_bind(wl_client* client, void* data, uint32_t version,
                             uint32_t id) {
    wl_resource* resource =
            wl_resource_create(client, &wl_shell_interface, version, id);
    assert(resource);

    auto self = static_cast<Shell*>(data);
    wl_resource_set_implementation(resource, &shell_interface, self,
                                   nullptr);
}

void Shell::get_shell_surface(wl_client *client, wl_resource *resource, uint32_t id, wl_resource *surface) {
}
