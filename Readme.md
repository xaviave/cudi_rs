# CUDI

### Description

CUDI is for Custom Diaporama.
This app is a display of the world, it will show you images with a particular aesthetic to keep your attention on the screen.

---

## Installation

- Never forget to update your system:

      sudo apt update && sudo apt upgrade

- Install [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) if it's not already done

- Install PostgreSQL:

   - Linux:

         sudo apt install postgresql postgresql-client

         echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env

   - Windows:

      Download PostgreSQL from the website and add the `lib` and `bin` path to your env path

            $Env:Path += ';C:\Program Files\PostgreSQL\[version]\bin;C:\Program Files\PostgreSQL\[version]\lib;

  Create the database:

      CREATE ROLE gmx;
      CREATE DATABASE cudi_db;
      CREATE USER gmx WITH PASSWORD '1234';
      ALTER ROLE gmx SET default_transaction_isolation TO 'read committed';
      GRANT ALL PRIVILEGES ON DATABASE cudi_db TO gmx;
      \q

- Run following commands:

      cd media_handler
      diesel migration run
      cd ..
      python3 toolbox.py -fc/--from-scratch

---

## To do

### Steps

1. Simple OpenGL engine in Rust:
   - ~~Open a window with some images shown~~
   - ~~Handle the high volume of local image to download per-second~~
   - 3D engine + CUDI as texture (WIP)
   - Handle the sound
   - Handle video on screen
2. Options and customizations of the engine:
   - Custom parameter to control the engine
     - ~~FPS~~
     - Control with mouse and Keyboard (WIP)
     - Filter
     - Mode (Cinema/Background/image with cudi in a plain rect see data/readme)
     - ~~Media path selector~~
     - Tags (WIP)
     - Animations
   - ~~Custom parameter in UX~~
   - Show Music player
   - Show Music analysis window
   - Media history (like/dislike) -> search engine w/ postgresql (WIP)
3. External API (real-time data acquisition):
   - Download photos or videos from API such as Archillect, Tumblr ou Google Photos
   - Handle the Deezer or Youtube API to play music
4. ML everywhere:
   - Add auto tagging image/video with themes/colors
   - Add music tagging with themes/styles and BPM selector
   - Adapt music and image/video BPM and tags

---

### Main parts:

#### Image selector

- `DONE`: create a database to stock the link of the image.
- create an ImageLink class that get the link from Archillect or Google Photos or instagram feed (check the max requests)
- create an ML tag creator class using tensorflow to make a classification from all images
- found different sources to get the image's links

#### ML Tag

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

#### Screen handler

- `DONE`: create a Menu class that override the screen and manage all the options (image_speed - tags - mode - filter - media_root).
- create different mode (normal - gif surrounding the screen - animation (image or video where a part of the screen will aff cudi, the reste will be the template))
- add an history of every file with a tag search or color search

#### Music handler

- manage the audio from the computer
- analyze the sound's curb to adapt cudi's parameter
- add an ML class to adapt the tags from the audio

#### Animation

Animation could be add to the screen:

- TV screen (high speed video\* accelerated with b/w filter and speed cut)
- Images in line on the top of the screen, a slow dark wallscreen animation will be always display. Those images will describe a type of sound (frequencies). When this type will appear in a music the corresponding image will grow then retake is place. All the sounds will create a nearly chaotic and unique animation for each music.
- Color palette with gradient from left to right in high speed
- Fill the screen with the same image in high speed and in a geometric way
- Themes displayed with associative color palette (death -> dark)
- [pixel sorting](<http://satyarth.me/articles/pixel-sorting/#:~:text=Pixel%20sorting%20is%20an%20interesting,(processing%20source%20code%20here).>)
- video in background and cudi on the top on certains part of the screen (anonymisation instead of blurring)
- check [compression artifact](https://en.wikipedia.org/wiki/Compression_artifact)
  [here also](https://github.com/scriptkittie/GlitchKernel)

##### \*car chase, illegal japanese drift, race (boat, bike, car, plane, spaceship)
