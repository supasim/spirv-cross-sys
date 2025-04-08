# Usage

I don't specify a minimum spirv cross version. Anything somewhat recent should work fine.

When using, you will have to specify the path of the libraries and the headers. When setting the `VULKAN_SDK` environment variable, these are automatically found.
When using this, it will search for headers in `$VULKAN_SDK/include/spirv-cross` subdirectory and libraries in `$VULKAN_SDK/lib` subdirectory.
You can also set the `SPIRV_CROSS_DIR` environment variable.
When using this, it will search for headers in the `$SPIRV_CROSS/include` subdirectory and libraries in the `$SPIRV_CROSS/lib` subdirectory.
Directory names are capitalized on windows.
Finally, you can set the `SPIRV_CROSS_HEADERS_DIR` and `SPIRV_CROSS_LIBS_DIR` environment variables, which do as you might guess.