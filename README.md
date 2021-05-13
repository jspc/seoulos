# SeoulOS Core

Core component of the SeoulOS kernel.

For a series of poorly spelled documents explaining what this does and why, see:

1. [SeoulOS original thoughts and notes](https://gist.github.com/jspc/968992020a9bbc61427a8fc95242ae4e)
1. [SeoulOS architecture redux: actual architecture](https://gist.github.com/jspc/2b80a03d5e1a09512cb36ee9b836e1cb)
1. [The SeoulOS filesystem](https://gist.github.com/jspc/ad5326040e0d25991e7a9a83bfd1f003)


## But essentially...

SeoulOS is a toy kernel. It uses a producer/consumer pattern to implement a modular microkernel. Core handles memory, processes, and files and plugins handle everything else, such as networking, filesystems, security, random bits of hardware.

The idea behind this is to write as little hard-to-test kernel code as possible, pushing as much as possible into small modular plugins. It's also to allow kernel code to be written in, basically, any language which can read/write the datatypes our kernel uses to communicate with plugins.

There are some side-effect goals, as it were. Some of the filetypes we use in other OSes aren't always as well suited to tasks as we'd like- it'd be nice to have transparent append-only support (as in, all writes, whether a file is opened in append mode or not, are appended without error), or tagging a file as important (so reclaimation jobs ignore it, or backup tasks know to back them up).

Likewise it'd be nice to be able to do other things and see whether they work. Can we have an operating whose libc is written in something like rust? Hell, could a whole memory allocator in rust make it harder for one process to get access to the memory of another process? Or even from causing buffer overflows?
