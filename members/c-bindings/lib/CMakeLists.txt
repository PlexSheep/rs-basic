cmake_minimum_required(VERSION 3.9)
project(test VERSION 1.0.1 DESCRIPTION "test lib")
include(GNUInstallDirs)
add_library(test SHARED test.c)
set_target_properties(test PROPERTIES
    VERSION ${PROJECT_VERSION}
    SOVERSION 1
    PUBLIC_HEADER test.h)
# configure_file(test.pc.in test.pc @ONLY)
target_include_directories(test PRIVATE .)
install(TARGETS test
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    PUBLIC_HEADER DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
install(FILES ${CMAKE_BINARY_DIR}/test.pc
    DESTINATION ${CMAKE_INSTALL_DATAROOTDIR}/pkgconfig)
