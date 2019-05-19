#include <assert.h>
#include "extreme-blend/reflector.h"
#include "shell.h"
#include "compositor.h"
#include "shell.h"
#include <gflags/gflags.h>
#include <glog/logging.h>
#include <gtest/gtest.h>
#include <condition_variable>

bool egl_init(wl_display *wl_disp);

class DisplayLoop {
    wl_display* display;
public:
    DisplayLoop(int terminate_readable_fd): display(NULL) {
        wl_log_set_handler_server([](const char* fmt, va_list args) {
            vprintf(fmt, args);
            LOG(INFO) << fmt;
        });
        display = wl_display_create();
        if (!display) {
            LOG(ERROR) << "Failed to create display";
        }

        if (!egl_init(display)) {
            LOG(INFO) << "Failed to init egl";
        }

        const char *socket_name = wl_display_add_socket_auto(display);
        if (!socket_name) {
            LOG(ERROR) << "Failed to create socket";
            return;
        }

        auto compositor = std::make_unique<Compositor>(display);
        auto shell = std::make_unique<Shell>(display);

        auto l = wl_display_get_event_loop((display));
        wl_event_loop_add_fd(l, terminate_readable_fd, WL_EVENT_READABLE, [](int, uint32_t, void* data){
            wl_display_terminate(reinterpret_cast<wl_display*>(data));
            return 0;
        }, display);

        LOG(INFO) << "Hello, Wayland";
    }

    void run() {
        if (!display) {
            LOG(ERROR) << "No display";
            return;
        }
        wl_display_run(display);
        wl_display_destroy(display);
    }
};

ExtremeBlend::Reflector::~Reflector() {
    char terminate_message[] = "terminate";
    write(terminate_writable_fd, terminate_message, strlen(terminate_message));
    loop_thread.join();
    close(terminate_readable_fd);
    close(terminate_writable_fd);
}

ExtremeBlend::Reflector::Reflector(): terminate_readable_fd(-1), terminate_writable_fd(-1) {
    int fds[2] = {-1, -1};
    if (pipe(fds) == -1) {
        LOG(ERROR) << "failed to create pipe";
        return;
    }
    terminate_readable_fd = fds[0];
    terminate_writable_fd = fds[1];

    bool display_ready = false;
    std::mutex display_ready_mutex;
    std::condition_variable display_ready_cond;

    loop_thread = std::thread([&](){
        DisplayLoop display_loop(terminate_readable_fd);
        {
            std::lock_guard<std::mutex> lock(display_ready_mutex);
            display_ready = true;
            display_ready_cond.notify_all();
        }
        display_loop.run();
    });

    {
        std::unique_lock<std::mutex> lock(display_ready_mutex);
        display_ready_cond.wait(lock, [&]{
            return display_ready;
        });
    }
}
