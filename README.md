# dcrawl

USING bevy 0.14
Experimenting with Bevy to create a dungeon crawler game.

TODO:
just having two cameras spawned, one for introscreen, one for mainmenu, caused the camera order ambiguity.
1. Have just a 2d camera spawn at game setup, maybe separate plugin - UI render should be to this camera
2. Have a 3d camera spawn at game runtime.
ORDER OF 2d CAMER SHOULD BE HIGHER THAN 3d CAMER
