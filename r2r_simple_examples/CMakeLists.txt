cmake_minimum_required(VERSION 3.5)
project(r2r_simple_examples)

find_package(ament_cmake REQUIRED)

if(NOT DEFINED CMAKE_SUPPRESS_DEVELOPER_WARNINGS)
     set(CMAKE_SUPPRESS_DEVELOPER_WARNINGS 1 CACHE INTERNAL "No dev warnings")
endif()

include(r2r_cargo.cmake)

r2r_cargo(std_msgs
          std_srvs
          rcl
          rcl_action
         )

if(WIN32)
  set(SUFFIX ".exe")
else()
  set(SUFFIX "")
endif()

install(PROGRAMS
  ${CMAKE_SOURCE_DIR}/target/colcon/pub${SUFFIX}
  DESTINATION lib/${PROJECT_NAME}
)

install(PROGRAMS
  ${CMAKE_SOURCE_DIR}/target/colcon/sub${SUFFIX}
  DESTINATION lib/${PROJECT_NAME}
)

install(PROGRAMS
  ${CMAKE_SOURCE_DIR}/target/colcon/service${SUFFIX}
  DESTINATION lib/${PROJECT_NAME}
)

install(PROGRAMS
  ${CMAKE_SOURCE_DIR}/target/colcon/client${SUFFIX}
  DESTINATION lib/${PROJECT_NAME}
)

ament_package()
