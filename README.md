# mandelbrot
~~im not going to tell you how to use it >:(~~\
i guess i will now :)

## general things
- Edit the const values in each file to edit location and zoom in the set (the bigger the `ZOOM` const the higher the zoom).
I find that [this](http://www.cuug.ab.ca/dewara/mandelbrot/) website has values that roughly correspond to the values that can be used in my program
(just do `1.0 / zoom value in website` for zoom)
- Edit the size const to change the size of the image/window rendered (500 is recommended)
- Make sure to run as release otherwise performance is terrible

## different bins
### main.rs
My first rendition of the set. Renders it live in an sdl2 window. The slowest and messiest one

### refactored_main.rs
Similar-ish performance to main but with cleaner code and without live rendering (only saving to image afterwards)

### parallel_main.rs
Uses rayon's `par_iter` for much faster performance (3-5x on my machine) along with a few other optimisations\
The recommended one to use

### julia.rs
Exactly the same as refactored_main.rs except it renders the [julia set](https://en.wikipedia.org/wiki/Julia_set) as opposed to the mandelbrot set