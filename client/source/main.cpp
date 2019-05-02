#include <stdio.h>
#include <wayland-client.h>

#ifndef ULTIMATE_DESKTOP_CLIENT_CUSTOM_MAIN

int main(void) {
    struct wl_display *display;

    display = wl_display_connect(NULL);
    if(!display)
    {
        printf("can not connect\n");
        return 1;
    }

    printf("connect\n");

    wl_display_disconnect(display);

    printf("disconnect\n");

    return 0;
}

#endif
