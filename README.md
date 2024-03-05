# Bevy Apps

The goal of this repository is to make apps using bevy.

### Table of content:
- [Uno game](#uno-game)

## Uno Game

<a name="uno-game"></a>

This app aims to create a uno game using Bevy.

### Cards
The first component I'm going to add to the app is the `Card` one. It holds information about the card color and which variant it is.
It's `impl`ementation allows to retrieve texture path based on the component info inside the asset folder. `Impl`emented `Into<&'static str>` to facilitate path retrieval.
