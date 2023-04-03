# CUDI

### Description

CUDI is for Custom Diaporama.
This app is a display of the world, it will show you images with a particular aesthetic to keep your attention on the screen.

---

## To do

---

### Steps

1. Simple OpenGL engine in Rust:
   - ~~Open a window with some images shown~~
   - Handle the high volume of local image to download per-second (WIP)
   - Handle the sound
   - Handle video on screen
2. Options and customizations of the engine:
   - Custom parameter to control the engine
     - ~~FPS~~
     - Filter
     - Mode (Cinema/Background)
     - ~~Media path selector~~
     - Tags
     - Animations
   - Custom parameter in UX
   - Show Music player
   - Show Music analysis window
   - Media history (like/dislike) -> search engine w/ postgresql
3. External API (real-time data acquisition):
   - Download photos or videos from API such as Archillect, Tumblr ou Google Photos
   - Handle the Deezer or Youtube API to play music
4. ML everywhere:
   - Add auto tagging image/video with themes/colors
   - Add music tagging with themes/styles and BPM selector
   - Adapt music and image/video BPM and tags

---

##### Image selector

- create a database to stock the link of the image
- create an ImageLink class that get the link from Archillect or Google Photos (check the max requests)
- create an ML tag creator class using tensorflow to make a classification from all images
- found different sources to get the image's links

##### ML Tag

Image tagger to reference images or videos following recurrent themes or aesthetic descriptions.
The tags could be stored within a SQL database next to the image link.

Find tag that referenced this themes:

- Main Colors
- Feelings
- Vibes / mood
- Art mouvements
- Words
- Custom tag to create KNN like media groups
- Music style

##### Screen handler

- create a Menu class that override the screen and manage all the options (image_speed - tags - mode - filter - media_root)
- create different mode (normal - gif surrounding the screen - animation (image or video where a part of the screen will aff cudi, the reste will be the template))
- add an history of every file with a tag search or color search

##### Music handler

- manage the audio from the computer
- analyze the sound's curb to adapt cudi's parameter
- add an ML class to adapt the tags from the audio

---

### Animation

Animation could be add to the screen:

- TV screen (high speed video\* accelerated with b/w filter and speed cut)
- Images in line on the top of the screen, a slow dark wallscreen animation will be always display. Those images will describe a type of sound (frequencies). When this type will appear in a music the corresponding image will grow then retake is place. All the sounds will create a nearly chaotic and unique animation for each music.

\*car chase, illegal japanese drift, race (boat, bike, car, plane, spaceship)
