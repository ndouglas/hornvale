# Architecture

This document describes the high-level architecture of ***Hornvale***.

See also the [Book](https://ndouglas.github.io/hornvale/), which is a more aspirational document about what I want to accomplish and how I want to get there, and the [README](./README.md), which is a _very_ basic introduction to the project.

## Bird's Eye View

_Hornvale_ will become a game that accepts semi-natural-language input from a player and supplies output in the form of descriptive text about a fictional world β in short, a type of game variously referred to as a "text adventure" or "interactive fiction".

Text adventures and interactive fiction are normally concise, tightly-crafted works, although they vary substantially in scope and scale.  _Hornvale_ differs from these in that it tries to be a an "open world," "procedurally generated" text adventure.

I'm not under any illusions about how complex a task this is.  I expect that if I'm able to bring it to full fruition, it will only be after a period of several years, possibly decades.  It's that concern that guides the various architectural and design decisions; I'm aiming for maintainability and testability and debuggability pretty much _ΓΌber alles_.

## Code Map

The following should be a 1:1 listing of the code in the `src/` subdirectory.

- [πͺ² Macros](./src/_macros/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Macros developed for various uses.  Given the peculiarities of how macros are made available in Rust, I've segregated them to a specific folder.

- [π­ Traits](./src/_traits/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Traits to ease dependency injection, genericity, decoupling code, etc.  I want to keep traits separate from the things assuming them as well as the things consuming them. 

- [π¬ Actions](./src/actions/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Actions, in-game actions undertaken by entities with the intent to alter something about the game world.

- [π§ββοΈ Anatomy](./src/anatomy/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Anatomy and physiology, health, damage, poison, and regeneration.

- [π« Astronomy](./src/astronomy/README.md)&nbsp;<sup><sub><sub>π </sub></sub></sup>: Astronomical sciences, from the galaxy to the moon.

- [π§¬ Biology](./src/biology/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Biological, taxonomy and related subjects.

- [πͺ¦ Combat](./src/combat/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Combat system, some closely related topics.

- [β Commands](./src/commands/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: The input from the player is parsed and transformed into commands, which should map to various in-character actions or out-of-character queries of or modifications to the game state.

- [π§© Components](./src/components/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Attributes and behaviors are shifted out of the Entity class and into small, focused types to promote composition and modularity.

- [π§­ Direction](./src/direction/README.md)&nbsp;<sup><sub><sub>π‘</sub></sub></sup>: Direction is a fundamental concept to navigation and description.

- [βοΈ Downdelving](./src/downdelving/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Dungeons, mining, and the Underdark.  Underground portions of the game pose different challenges than above-ground portions, and we deal with some of those issues here.

- [π¦ Economics](./src/economics/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Economic system, merchants, resources, scarcity.  I want shopkeepers to actually consider economic ideas, rather than act as a sink for currency.

- [π₯ Effects](./src/effects/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Alterations to the world state should take certain pre-defined forms that can be tested for accuracy and correctness.

- [π½ Entity](./src/entity/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Entities are any objects that appear in the game.  This section comprises not so much entity _behavior_ (which should be handled primarily in the Components), but the creation and management of entities.

- [π? Game](./src/game/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Game state management.

- [π Geology](./src/geology/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Geology and physical geography, for terrain generation at a macro and local scale.

- [π§  Goal-Oriented Action Planning](./src/goap/README.md)&nbsp;<sup><sub><sub>π </sub></sub></sup>: When an entity selects a goal, this system can be used to select the action that they should take to move toward accomplishing it.

- [π Input/Output](./src/io/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Input/Output interfaces are centralized so that we can easily adapt to new systems, e.g. playing through a telnet connection rather than running the application directly.

- [π¬ Linguistics](./src/linguistics/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Linguistics system permitting communication, bidirectional translation, etc.

- [πΊοΈ Map](./src/map/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: The representation of the game world with regard to its most fundamental navigational components: rooms.

- [π Mythopoetics](./src/mythopoetics/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Narrative/mythopoetic procedural content generation and tools.  The end goal is being able to generate _interesting_ stories, plotlines, etc.

- [π Parsing](./src/parsing/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Parsing user input and transforming it into commands of arbitrary complexity.

- [πͺ Passage](./src/passage/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Passages can be somewhat complex; either hallways, doorways, lockable doors, hidden exits, not-very-visible exits, slow exits, exits that present a message when the user attempts to go in that direction, etc.

- [ποΈ Perception](./src/perception/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Stimuli are processed by senses and perceived by individual entities.

- [βΉοΈ Player](./src/player/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: I try to universalize as much as possible of behavior between player-characters and non-player-characters, but there is _some_ stuff that is particular to the player.

- [π Room](./src/room/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: The Room concept and related tools.

- [π» Scripting Language](./src/scripting/README.md)&nbsp;<sup><sub><sub>π‘</sub></sub></sup>: Embedded programming language (based on [Lox](https://www.craftinginterpreters.com/)) and domain-specific library.

- [π§βπ€βπ§ Sociology](./src/sociology/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Social psychology and sociology, individual and group behavior.

- [π» Supernatural](./src/supernatural/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Supernatural and metaphysical concepts, the thermodynamics of spirit.

- [πΏ User Interface](./src/ui/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Frontend and connective logic.  As little as possible.

- [π World](./src/world/README.md)&nbsp;<sup><sub><sub>π΄</sub></sub></sup>: Data structure containing everything that happens within the in-game world.

**Status**: These indicators' meanings are subject to change as I progress.
 - <sup><sub><sub>π΄</sub></sub></sup>: I haven't even started.
 - <sup><sub><sub>π </sub></sub></sup>: I've laid the groundwork, or at least taken some initial steps.
 - <sup><sub><sub>π‘</sub></sub></sup>: It's serving some purpose, though far from complete.
 - <sup><sub><sub>π’</sub></sub></sup>: Working, although I'll never really consider it "feature complete".
 - <sup><sub><sub>π΅</sub></sub></sup>: A vast radiant beach and cool jeweled moon, etc.  Some evenings I just watch the test suites as they run.
