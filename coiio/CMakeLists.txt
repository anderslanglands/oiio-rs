cmake_minimum_required(VERSION 3.5)
project(coiio)

set(CMAKE_CXX_STANDARD 14)

add_library(coiio STATIC coiio.cpp)
target_include_directories(coiio PUBLIC ${INC_OIIO} ${INC_OPENEXR})
target_link_libraries(coiio PUBLIC 
    ${LIB_OIIO} 
    ${LIB_ILMIMF} 
    ${LIB_IMATH} 
    ${LIB_IEX} 
    ${LIB_HALF})
target_compile_options(coiio PRIVATE "-Wno-deprecated-register" "-D_GLIBCXX_USE_CXX11_ABI=0")

install(TARGETS coiio DESTINATION ${CMAKE_INSTALL_PREFIX})

