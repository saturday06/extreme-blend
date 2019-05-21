#include "extreme-blend/reflector.h"
#include "compositor.h"
#include "shell.h"
#include "zxdg_shell_v6.h"
#include <assert.h>
#include <condition_variable>
#include <gflags/gflags.h>
#include <glog/logging.h>
#include <gtest/gtest.h>

bool egl_init(wl_display *wl_disp);

class DisplayLoop {
  std::unique_ptr<Compositor> compositor;
  std::unique_ptr<Shell> shell;
  std::unique_ptr<ZxdgShellV6> zxdg_shell_v6;
  std::unique_ptr<wl_display, decltype(&wl_display_destroy)> display; // last
public:
  DisplayLoop(int terminate_readable_fd)
      : display(wl_display_create(), wl_display_destroy) {
    if (!display) {
      LOG(ERROR) << "Failed to create display";
    }

    if (!egl_init(display.get())) {
      LOG(INFO) << "Failed to init egl";
    }

    const char *socket_name = wl_display_add_socket_auto(display.get());
    if (!socket_name) {
      LOG(ERROR) << "Failed to create socket";
      display.reset();
      return;
    }

    compositor = std::make_unique<Compositor>(display.get());
    shell = std::make_unique<Shell>(display.get());
    zxdg_shell_v6 = std::make_unique<ZxdgShellV6>(display.get());

    auto l = wl_display_get_event_loop((display.get()));
    wl_event_loop_add_fd(
        l, terminate_readable_fd, WL_EVENT_READABLE,
        [](int, uint32_t, void *data) {
          wl_display_terminate(reinterpret_cast<wl_display *>(data));
          return 0;
        },
        display.get());

    LOG(INFO) << "Hello, Wayland";
  }

  void run() {
    if (!display) {
      LOG(ERROR) << "No display";
      return;
    }
    wl_display_run(display.get());
  }
};

void ExtremeBlend::Reflector::wait_for_exit() { loop_thread.join(); }

ExtremeBlend::Reflector::~Reflector() {
  char terminate_message[] = "terminate";
  write(terminate_writable_fd, terminate_message, strlen(terminate_message));
  loop_thread.join();
  close(terminate_readable_fd);
  close(terminate_writable_fd);
}

ExtremeBlend::Reflector::Reflector()
    : terminate_readable_fd(-1), terminate_writable_fd(-1) {
  int fds[2] = {-1, -1};
  if (pipe(fds) == -1) {
    LOG(ERROR) << "failed to create pipe";
    return;
  }
  terminate_readable_fd = fds[0];
  terminate_writable_fd = fds[1];

  std::mutex display_ready_mutex;
  std::condition_variable display_ready;

  loop_thread = std::thread([&]() {
    DisplayLoop display_loop(terminate_readable_fd);
    display_ready.notify_one();
    display_loop.run();
  });

  {
    std::unique_lock<std::mutex> lock(display_ready_mutex);
    display_ready.wait(lock);
  }
}
