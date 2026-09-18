# Notes

## Learner profile
- Experienced programmer (self-taught, productive in TypeScript/JS — see /typescript workspace)
- Comfortable with general programming concepts: functions, types, control flow, data structures
- New to Odin, new to graphics/games programming
- Learning style observed in TypeScript sessions: moves fast, terse feedback ("great. next"), prefers short lessons with one win per session

## Teaching preferences
- Skip general programming fundamentals; focus on what is Odin-specific or graphics-specific
- Short lessons, tight quiz feedback loops
- Opted out of communities in TypeScript sessions — assume same here unless asked

## Target project
Joust clone (1982 arcade game). Core mechanics: gravity + flap physics, screen wrapping, platform collision, lance-height combat, enemy state machine (flying → egg → hatching → harder enemy), wave progression. Single-player first.

## Lesson arc (Joust-oriented)
Each lesson leads directly toward one Joust mechanic.

| # | Topic | Joust relevance |
|---|-------|----------------|
| 01 | Hello, Odin | ✓ done |
| 02 | Variables & Types | ✓ done |
| 03 | Control flow | if/for/switch — game loop conditionals, AI branches |
| 04 | Procedures | functions, multiple return, or_return — game update functions |
| 05 | First window | raylib: open a window, clear screen, 60fps loop |
| 06 | Drawing shapes | rectangles for platforms, circles for player placeholder |
| 07 | Physics — gravity & flap | vertical velocity, gravity constant, flap impulse |
| 08 | Horizontal movement & screen wrap | left/right input, modular wrap |
| 09 | Platform collision | AABB test, landing on ledges |
| 10 | Sprites | load texture, draw sprite, flip direction |
| 11 | Enemies | struct + state enum, patrol AI, spawn positions |
| 12 | Lance combat | hitbox height comparison, egg spawning on death |
| 13 | Egg & hatch system | timer-based hatch, collect-for-points |
| 14 | Waves & score | wave counter, enemy count, score display, lives |
| 15 | Polish | lava, pterodactyl (bonus enemy), game-over screen |

## Open questions
- Preferred cadence (sessions/week)?
