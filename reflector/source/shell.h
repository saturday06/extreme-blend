#pragma once

#include <wayland-server.h>

class Shell {
public:
    static void get_shell_surface(wl_client *client, wl_resource *resource, uint32_t id, wl_resource *surface);
};
