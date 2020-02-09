# Distributed under the OSI-approved BSD 3-Clause License.  See accompanying
# file Copyright.txt or https://cmake.org/licensing for details.

cmake_minimum_required(VERSION 3.16.0)

# We name the project and the target for the ExternalProject_Add() call
# to something that will highlight to the user what we are working on if
# something goes wrong and an error message is produced.

project(al2o3_platform-populate NONE)

include(ExternalProject)
ExternalProject_Add(al2o3_platform-populate
                     "UPDATE_DISCONNECTED" "False" "GIT_REPOSITORY" "https://github.com/Deanoc/al2o3_platform" "GIT_TAG" "master"
                    SOURCE_DIR          "C:/Users/Computer/Documents/Code/beany/al2o3/al2o3_platform-src"
                    BINARY_DIR          "C:/Users/Computer/Documents/Code/beany/al2o3/al2o3_platform-build"
                    CONFIGURE_COMMAND   ""
                    BUILD_COMMAND       ""
                    INSTALL_COMMAND     ""
                    TEST_COMMAND        ""
                    USES_TERMINAL_DOWNLOAD  YES
                    USES_TERMINAL_UPDATE    YES
)
