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
