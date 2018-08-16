A simple top-scrolling shmup, written in rust using piston.

Controlled with WASD for movement and Spacebar to shoot.

Began with the the piston spinning square example to set up the window and core game loop, then started design with a
loosely unity like abstraction, all the moving parts were built to implement a core 'Entity' trait, and were stored in 
a Vec of boxed trait objects to begin.   

following that was setting up the player, projectiles, and enemies; figuring out how to handle input events to achieve 
the necessary behavior. Aside from having to store the key state for all of the inputs since key repeat lead to some 
unexpected and inconsistent input events, this was fairly straightforward. Several functions for entities were able to 
be implemented in the core trait, either exclusively or with useful defaults, making the abstraction very useful.

Next I came to my first real tango with the borrow checker, trying to get multiple objects in the entity collection to 
interact. Quickly I was able to figure out all of the non-mutating comparison work, and simply set aside indexes for the 
to do the mutation later, however I still needed information from the other for the mutation, so it mostly delayed the 
problem. finally, as I was preparing to start saving half the info in the structs, so I could take a single mutable 
reference at a time, I discovered that as with most structures in rust, it is not actually that hard to something 
provably safe as long as you can find the right function. In this case, split_at_mut allowed me two mutable references, 
since the two elements were always distinct. 

Later, I revisited this problem with Cells, allowing me to do away with building Vecs of indexes to interact in turn, 
hoping this would improve the performance (not that it's needed in this simple game), however the overhead of the Cells 
ultimately reduced the performance more than the extra memory overhead.

The final, and greatest challenge was something of a surprise to me, figuring out how to print text to the screen. I 
found numerous guides on the subject during my search, but even the documentation I found seemed to be out of date, or 
at the very least unclear, and I ended up spending basically an entire day trying to track down the right invocation, 
setting up, and passing around a glyph cache, etc. 

Currently, it is just a simple random spawning high score challenge, because I don't have the time to try to design any 
levels, or add menus, but it is now mostly a process of game design rather than dealing with the language.

There are a couple issues, one of which I discovered only yesterday. the first, is that the window backend currently 
ignores the resizable = false flag, and for simplicity, everything currently assumes the window size doesn't change, 
so resizing the window leads to dead space, or cut off game area. And secondly, I just discovered that either the delta 
time or the update ticks are wildly inconsistent from one machine to another, leading to dramatically different play 
experiences on some machines.
