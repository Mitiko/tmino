![alt text](https://github.com/Mitiko/tmino/blob/main/solution-6-23x24.jpeg?raw=true)

# T-mino solution finder

This project tries to fill a grid MxN with polyminos using backtracking (smart-ish bruteforce).

The algorithm is applicable to any polymino and can be modified to different shapes grids
but the base version of it uses a specific t-polymino of size 6.

## How to draw the solution

If a solution is found, an image solution-****.bmp will be generated where each pixel in the bitmap is a cell in the grid.  
To upscale the image you can use imagemagick with `convert in.bmp -filter point -resize 230x240 out.jpg` and you can substitute the desired image dimensions. (Cool trick - you can get some cool blurry art if you omit -filter point).

## How to change the polymino

There's a macro parser (running compile time) to generate code for the different rotations and flips of polyminos.
You could easily change it to a different polymino as long as you put (0, 0) in the top left cell of the polymino.

You should replace the sequences of cells in `try_fill`, `unfill`, and `apply_stack`.
Another function to mind is the `get_gap_index` which returns the first empty cell in the grid starting from a given position (by default the last inserted polymino).  
Any function that returns an empty cell should work but different implementations will vary in speed of solver.
