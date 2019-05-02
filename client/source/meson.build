project('learning-wayland', 'cpp', 'c', default_options : ['cpp_std=c++14'])

wl_client = dependency('wayland-client')
egl = dependency('egl')
glesv2 = dependency('glesv2')
x11 = dependency('x11')
xcb = dependency('xcb')
x11_xcb = dependency('x11-xcb')

executable('client', 'main.cpp', dependencies: [wl_client, egl, glesv2, x11, xcb, x11_xcb])
