# Mission: Build a Joust Clone in Odin

## Why
Build a complete, playable clone of the 1982 arcade game Joust using Odin and raylib — a knight on an ostrich, gravity-based flap mechanics, lance-height jousting, enemy buzzard riders, and wave-based progression. The target is something genuinely fun to play, not just a tech demo.

## Success looks like
- A window opens at a fixed frame rate with a game loop running
- A player knight on an ostrich with gravity pulling down and flapping pushing up
- Platform collision — land on ledges, fall off edges
- Screen wrapping — fly off the left edge, appear on the right
- Enemies: buzzard riders that patrol platforms at varying speeds
- Lance-height combat: the higher lance wins; loser becomes an egg
- Eggs can be collected for points; if left too long, they hatch into a harder enemy
- Score display, lives counter, wave number
- Wave clears when all enemies are defeated; next wave spawns more/harder enemies
- Something recognisably Joust that is fun to play for five minutes

## Constraints
- Experienced programmer — skip general programming concepts
- New to Odin and new to graphics/games — build up from first principles
- 2D only; no shaders or 3D
- Scope to a single-player game first (two-player is a stretch goal)

## Out of scope (for now)
- Original arcade graphics / ROM assets — placeholder shapes and sprites are fine
- Sound and music (add later once the game loop is solid)
- Two-player co-op
- Lava troll, pterodactyl, and other advanced Joust enemies (add after core loop works)
