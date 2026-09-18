package main

import rl "vendor:raylib"

SCREEN_W :: 800
SCREEN_H :: 600
GRAVITY :: f32(800)
FLAP_FORCE :: f32(420)
MAX_FALL :: f32(600)
MOVE_SPEED :: f32(200)
PLAYER_W :: f32(40)
PLAYER_H :: f32(60)

PLATFORMS :: [?]rl.Rectangle {
	{0, 554, 280, 46},
	{520, 554, 280, 46},
	{48, 426, 176, 18},
	{576, 426, 176, 18},
	{288, 316, 224, 18},
	{336, 186, 128, 18},
}

LAVA :: rl.Rectangle{280, 574, 240, 26}

make_rider_sprite :: proc() -> rl.Texture2D {
	img := rl.GenImageColor(40, 60, rl.BLANK)
	rl.ImageDrawRectangle(&img, 6, 28, 28, 22, rl.ORANGE)
	rl.ImageDrawRectangle(&img, 12, 18, 16, 14, rl.ORANGE)
	rl.ImageDrawRectangle(&img, 10, 8, 20, 14, rl.DARKBROWN)
	rl.ImageDrawRectangle(&img, 14, 2, 12, 10, rl.BROWN)
	rl.ImageDrawRectangle(&img, 30, 13, 10, 3, rl.LIGHTGRAY)
	t := rl.LoadTextureFromImage(img)
	rl.UnloadImage(img)
	return t
}

main :: proc() {
	rl.InitWindow(SCREEN_W, SCREEN_H, "Joust")
	defer rl.CloseWindow()
	rl.SetTargetFPS(60)

	rider := make_rider_sprite() // after InitWindow!
	defer rl.UnloadTexture(rider)

	player_x: f32 = 380
	player_y: f32 = 256
	player_vx: f32 = 0
	player_vy: f32 = 0
	player_facing: f32 = 1

	for !rl.WindowShouldClose() {
		// time in seconds for last frame drawn
		dt := rl.GetFrameTime()

		// direction change
		if rl.IsKeyDown(.LEFT) {
			player_vx = -MOVE_SPEED
			player_facing = -1
		} else if rl.IsKeyDown(.RIGHT) {
			player_vx = MOVE_SPEED
			player_facing = 1
		} else {
			player_vx = 0
		}

		// vertical physics
		player_vy += GRAVITY * dt
		player_vy = min(player_vy, MAX_FALL)
		if rl.IsKeyPressed(.SPACE) {
			player_vy -= FLAP_FORCE
		}

		// integration
		player_x += player_vx * dt
		player_y += player_vy * dt

		// player collision detection
		player_rect := rl.Rectangle{player_x, player_y, PLAYER_W, PLAYER_H}
		for plat in PLATFORMS {
			if rl.CheckCollisionRecs(player_rect, plat) && player_vy > 0 {
				player_y = plat.y - PLAYER_H
				player_vy = 9
			}
		}

        
		if player_x > f32(SCREEN_W) {player_x = -PLAYER_W}
		if player_x < -PLAYER_W {player_x = f32(SCREEN_W)}

		if player_y > f32(SCREEN_H) {player_y = -PLAYER_H}
		if player_y < -PLAYER_H {player_y = f32(SCREEN_H)}

        if player_y < 0 {
            player_vy = 9
        }

		rl.BeginDrawing()

		rl.ClearBackground(rl.BLACK)

		rl.DrawRectangleGradientV(
			i32(LAVA.x),
			i32(LAVA.y),
			i32(LAVA.width),
			i32(LAVA.height),
			rl.RED,
			rl.ORANGE,
		)

		for plat in PLATFORMS {
			rl.DrawRectangleRec(plat, rl.DARKGRAY)
		}

		// draw rider sprite, flipped when facing left
		src_x := f32(0) if player_facing > 0 else f32(rider.width)
		src_w := f32(rider.width) * player_facing
		src := rl.Rectangle{src_x, 0, src_w, f32(rider.height)}
		dst := rl.Rectangle{player_x, player_y, PLAYER_W, PLAYER_H}
		rl.DrawTexturePro(rider, src, dst, {0, 0}, 0, rl.WHITE)

		rl.DrawText("JOUST", 10, 10, 20, rl.YELLOW)

		rl.EndDrawing()
	}
}
