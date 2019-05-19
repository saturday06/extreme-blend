#include "zxdg_shell_v6.h"
#include "zxdg_surface_v6.h"

#include <cassert>
#include <cstdio>
#include <xdg-shell-unstable-v6-server-header.h>

static struct zxdg_shell_v6_interface shell_interface = {
    &ZxdgShellV6::destroy, &ZxdgShellV6::create_positioner, &ZxdgShellV6::get_xdg_surface,
    &ZxdgShellV6::pong};

ZxdgShellV6::ZxdgShellV6(wl_display* display)
    : display_(display) {
    wl_global_create(display_, &zxdg_shell_v6_interface, 1, this,
                     &ZxdgShellV6::global_bind);
}

void ZxdgShellV6::global_bind(wl_client* client, void* data, uint32_t version,
                        uint32_t id) {
    wl_resource* resource =
        wl_resource_create(client, &zxdg_shell_v6_interface, version, id);
    assert(resource);

    auto self = static_cast<ZxdgShellV6*>(data);
    wl_resource_set_implementation(resource, &shell_interface, self, nullptr);
}

void ZxdgShellV6::destroy(struct wl_client* client, wl_resource* resource) {
    fprintf(stderr, "ZxdgShellV6::destroy not implemented\n");
}

void ZxdgShellV6::create_positioner(wl_client* client, wl_resource* resource,
                              uint32_t id) {
    fprintf(stderr, "ZxdgShellV6::create_positioner not implemented\n");
}

void ZxdgShellV6::get_xdg_surface(wl_client* client, wl_resource* resource,
                            uint32_t id, wl_resource* surface) {
    auto shell_surface = new ZxdgSurfaceV6(client, id);
    assert(shell_surface);
}

void ZxdgShellV6::pong(wl_client* client, wl_resource* resource, uint32_t serial) {
    fprintf(stderr, "ZxdgShellV6::create_positioner not implemented\n");
}
