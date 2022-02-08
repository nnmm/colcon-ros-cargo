cmake_minimum_required(VERSION 3.5)
project(cargo-ros-install NONE)

find_package(ament_cmake REQUIRED)

configure_file(${CMAKE_SOURCE_DIR}/Cargo.toml ${CMAKE_BINARY_DIR}/Cargo.toml COPYONLY)
configure_file(${CMAKE_SOURCE_DIR}/src/lib.rs ${CMAKE_BINARY_DIR}/src/lib.rs COPYONLY)
configure_file(${CMAKE_SOURCE_DIR}/src/main.rs ${CMAKE_BINARY_DIR}/src/main.rs COPYONLY)
add_custom_command(
    OUTPUT
        ${CMAKE_BINARY_DIR}/target/release/cargo-ros-install
    COMMAND cargo build -q --release
    DEPENDS
        src/lib.rs
        src/main.rs
        Cargo.toml
    WORKING_DIRECTORY
        ${CMAKE_BINARY_DIR}
)

add_custom_target(
    build_crate ALL
    DEPENDS
        ${CMAKE_BINARY_DIR}/target/release/cargo-ros-install
)

install(FILES
    ${CMAKE_BINARY_DIR}/target/release/cargo-ros-install
    PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE
    DESTINATION bin
)
ament_package()