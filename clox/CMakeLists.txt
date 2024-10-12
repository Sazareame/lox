cmake_minimum_required(VERSION 3.15)

project(
  clox
  LANGUAGES C
  VERSION 0.1.0
)

add_executable(lox)
aux_source_directory(./src source)
target_sources(lox PUBLIC ${source})
target_include_directories(lox PUBLIC ./include)