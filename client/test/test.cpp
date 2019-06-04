#include <fcntl.h>
#include <glog/logging.h>
#include <gtest/gtest.h>
#include <stdlib.h>
#include <string>
#include <unistd.h>
#include <vector>
#include <wayland-server.h>

int main(int argc, char **argv) {
  if (setenv("WAYLAND_DEBUG", "1", 1)) {
    perror(argv[0]);
    return 1;
  }

  if (setenv("WAYLAND_DISPLAY", "/mnt/c/temp/temp.unixsock", 1)) {
    perror(argv[0]);
    return 1;
  }

  std::vector<char> xdg_runtime_dir_template;
  if (false) {
    std::unique_ptr<char, void (*)(char *)> cwd_ptr(
        get_current_dir_name(), [](char *data) { free(data); });
    if (!cwd_ptr) {
      perror(argv[0]);
      return 1;
    }
    std::string cwd(cwd_ptr.get());
    xdg_runtime_dir_template.insert(xdg_runtime_dir_template.end(), cwd.begin(),
                                    cwd.end());
    std::string dir = "/run-XXXXXX";
    xdg_runtime_dir_template.insert(xdg_runtime_dir_template.end(), dir.begin(),
                                    dir.end());
  } else {
    std::string tmp = "/tmp/eb-XXXXXX";
    xdg_runtime_dir_template.insert(xdg_runtime_dir_template.end(), tmp.begin(),
                                    tmp.end());
  }

  char *xdg_runtime_dir = mkdtemp(&xdg_runtime_dir_template[0]);
  if (xdg_runtime_dir == NULL) {
    perror(argv[0]);
    return 1;
  }
  if (setenv("XDG_RUNTIME_DIR", xdg_runtime_dir, 1)) {
    perror(argv[0]);
    return 1;
  }

  google::InitGoogleLogging(argv[0]);
  FLAGS_logtostderr = true;
  testing::InitGoogleTest(&argc, argv);

  LOG(INFO) << "XDG_RUNTIME_DIR=" << xdg_runtime_dir;
  return RUN_ALL_TESTS();
}

int os_create_anonymous_file(int size) {
  static const char fmt[] = "/shared-XXXXXX";
  const char *path = NULL;
  char *name = NULL;
  int fd = -1;
  path = getenv("XDG_RUNTIME_DIR");
  if (!path) {
    errno = ENOENT;
    return -1;
  }
  name = (char *)malloc(strlen(path) + sizeof(fmt));
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
