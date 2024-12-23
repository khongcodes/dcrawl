# dcrawl

## current version
USING bevy 0.15

## what is this
Experimenting with Bevy to create a dungeon crawler game.

## TODO:

1. break down dcrawler-bevy0.14
2. rebuild with changes into 0.15
3. work out camera system

just having two cameras spawned, one for introscreen, one for mainmenu, caused the camera order ambiguity.
1. Have just a 2d camera spawn at game setup, maybe separate plugin - UI render should be to this camera
2. Have a 3d camera spawn at game runtime.
ORDER OF 2d CAMER SHOULD BE HIGHER THAN 3d CAMER
