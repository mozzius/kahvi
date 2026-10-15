# libkahviruntimefactory.so: the JSI shim, the RN factory and kahvi's C
# library (libs/<abi>/libkahvi_ffi.a, built by scripts/build-android.sh).
cmake_minimum_required(VERSION 3.13)
project(kahviruntimefactory)

set(CMAKE_CXX_STANDARD 20)
string(APPEND CMAKE_CXX_FLAGS " -fexceptions -frtti -O3")

include("${REACT_NATIVE_DIR}/ReactAndroid/cmake-utils/folly-flags.cmake")
add_compile_options(${folly_FLAGS})

set(PACKAGE_NAME "kahviruntimefactory")
set(SRC_DIR "${CMAKE_SOURCE_DIR}/src/main/cpp")
set(COMMON_DIR "${CMAKE_SOURCE_DIR}/../cpp")
set(KAHVI_FFI "${CMAKE_SOURCE_DIR}/libs/${ANDROID_ABI}/libkahvi_ffi.a")
if(NOT EXISTS "${KAHVI_FFI}")
  message(FATAL_ERROR "${KAHVI_FFI} is missing: run react-native/scripts/build-android.sh")
endif()

add_library(kahvi_ffi STATIC IMPORTED)
set_target_properties(kahvi_ffi PROPERTIES IMPORTED_LOCATION "${KAHVI_FFI}")

add_library(
  ${PACKAGE_NAME}
  SHARED
  "${COMMON_DIR}/KahviJSI.cpp"
  "${COMMON_DIR}/KahviInstance.cpp"
  "${SRC_DIR}/OnLoad.cpp"
)

target_include_directories(
  ${PACKAGE_NAME}
  PRIVATE
  "${COMMON_DIR}"
  "${CMAKE_SOURCE_DIR}/../include"
  "${REACT_NATIVE_DIR}/ReactCommon"
  "${REACT_NATIVE_DIR}/ReactCommon/jsitooling"
  "${REACT_NATIVE_DIR}/ReactAndroid/src/main/jni"
)

find_library(LOG_LIB log)
find_package(fbjni REQUIRED CONFIG)
find_package(ReactAndroid REQUIRED CONFIG)

target_link_libraries(
  ${PACKAGE_NAME}
  ${LOG_LIB}
  android
  kahvi_ffi
  fbjni::fbjni
  ReactAndroid::jsi
  ReactAndroid::reactnative
)
