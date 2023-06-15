# Refactor the engine.

Right now, everything was build around the texture, it needs to be changed.
Re-use the current architecture where each buffer render in a texture and print it in a framebuffer texture.

1. clean the buffer
2. render the asked asset in a off-screen texture
3. repeat
4. check if the framebuffer need to be clean
5. render the texture in the framebuffer on screen

Add control on buffer rendering:

    	- position function
    	- static or modifiable
    	- texture used
    	- effect on screen
    	- 3d model or not
    	- better config file
