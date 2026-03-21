# Implementing a ray-tracing engine with Rust from scratch with the help of [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

# How to use:
- Open your command terminal and run `git clone https://github.com/TotoMC-13/barnacle-ray.git`
- `cd barnacle-ray` **important notice:** The run file only works for Windows as it's a `.bat` file, it's not hard to create one for Linux or whatever you use but at this point of the project I'm too lazy to do it, feel free to make a PR if you make one and I'll gladly merge it.
- `compile.bat` and wait for it to finish rendering
- Now in the barnacle-ray folder an `image.ppm` file has been generated, usually image viewers can't open these files, either
upload your file here [PPM File viewer](https://www.cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html), install some app that
is able to properly display the format or the option I chose, download an extension for your IDE, in my case I use [this one](https://open-vsx.org/vscode/item?itemName=ngtystr.ppm-pgm-viewer-for-vscode) by ngtystr

## Why can't I see my image?
- I had a small issue regarding the VS Code extension, the file **must** be UTF-8 encoded or the extension I use won't load it.