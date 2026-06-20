# Boids!

Boids are cellular automota that simulate flocking behavior of birds. It's a fairly simple algorithm that you can implement in a few hours, so it's perfect for learning new languages. Since I've been wanting to learn rust, this project exists. Rust is known for its efficiency, so it seemed right to me to make this implementation as fast as possible.

## The goal
I would like to be able to have a boid algorithm running for 100 000 boids on 60 fps.

## The algorithm
If you would like to know how the algorithm works, I highly recommend [this blogpost](https://vanhunteradams.com/Pico/Animal_Movement/Boids-algorithm.html). I basically follow it every time I implement boids. Do note that there is a "mistake": the boids get updated in the loop. This means that alignemnt is not accurate to the "snapshot" of all the boids in a certain iteration. 

Off course, mistake is relative: this is not a mathematical problem, but a pretty picture generator. I'm just a pedantic nerd. 

### some specifics
The code works as follows:
1. In main, do the game loop
2. this game loop calls `world.step`, a function defined in world.rs
3. in this function, we loop through all the boids and calculate the new speeds for all the boids.
  We write all these speeds in a `speeds` vec.
4. then we write out all the speeds to the actual boids
5. return and make raylib draw all the boids

## The optimizations
The initial implementation was unoptimized because 1. I just wanted to get something up and running and 2. I'm not that good at rust yet. In no particular order, here are some optimizations.

### Simplified the update struct
Going through the git history, you may find that the `speeds` vec used to be a `Forces` vec. `Forces` were a struct I defined tracking the seperation, alignment and cohesion separatly, after which in the update loop all these forces were applied to the boid, which then calculated its new speed.

This was a big struct (6 `f32`'s and 1 `usize` for the amount of neighbours found). After seeing [Andrew Kelley's talk about data oriented design](https://www.youtube.com/watch?v=IroPQ150F6c), it got me thinking about how I could save on space. It turned out to be much simpler from a developer experience perspective and more performant to just calculate the new speeds in the first loop. It feels somewhat shameful to admit that I didn't see this from the start, but if a solution is that obvious, it just means I've learned.

### Moved away from the n-squared solution
The simplest way to implement boids is to loop through all the boids, then for every boid loop through all the boids to find the neighbours. The first optimization you will come accross on the web for boids, is to use a grid structure or a quadtree.

I went with a gridstructure because construction of the quadtree seemed complicated and runtime expensive. The idea is really simple: divide the space into a grid with cell size the distance that distinguishes a neighbour from a stranger. Then, before you start calculating, you loop through all the boids and assign them to their correct cell. Since lookup time of the correct cell is constant, this is a lineair operation. Then, when you want to calculate all the forces for a boid, you only look into the boids cell/neighbouring cells.

Technically this is still quadratic, but I think we can all see that in this case big-O notation is a deceiving us and hiding all the important weights.

### Multithreading
Even though I'm used to writing go, I really like the way rust handles multithreading. The fact that you have to consider when an object can be mutated, and when its read only, makes concurrency, truly, fearless.

The big cost in this algorithm is calculating all the new speeds. Since the boids are read only in this loop, it's easy to distribute this work among multiple threads: just split the big boid vec into multiple chunks, and let each thread handle one of these chunks.

### Using textures
Instead of telling the GPU 100 000 times "hey make this pixel white", a texture was created, onto which all the boids are drawn. This dramatically reduces GPU calls, which makes everything much faster. 

## Running this algorithm
You may notice the justfile included in this project. In order to run this algorithm you can run `just run` (if you have `just` installed). If you don't have just installed, you can off course run the command in the justfile.

If you would like to mess around with the parameters, the `context.rs` file contains all the variables and args for this algorithm. Here you can change things like the amount of boids, the weights for every force, the vision distance...

 
