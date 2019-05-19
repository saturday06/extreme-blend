#ifndef $HEADER$_REFLECTOR_H
#define $HEADER$_REFLECTOR_H

#ifdef __cplusplus

#include <memory>
#include <thread>
#include <wayland-server.h>

namespace ExtremeBlend {
    class Reflector {
    public:
        Reflector();

        ~Reflector();

        Reflector(const Reflector &) = delete;

        Reflector &operator=(const Reflector &) = delete;

    private:
        std::thread loop_thread;
        int terminate_readable_fd;
        int terminate_writable_fd;
    };
}

#endif

#endif
