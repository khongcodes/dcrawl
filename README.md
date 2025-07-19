# dcrawl

## current version
USING bevy 0.15

## what is this
Experimenting with Bevy to create a dungeon crawler game.

## TODO:

finish implementing GameState inline with flow diagram
remove ingame_menu as a state

- [x] BASIC state-UI specs defined
- [ ] remove ingame_menu from model and reexport diagram
    - (BUT InGameState should have a MainMenuButton)

InGame menu should be a substate of InGame - entering the InGame menu should not trigger CLEANUP of all rendered InGame entities!

Maybe something like a camera switch?
