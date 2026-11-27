#!/usr/bin/env python3
import curses
import random

def main(stdscr):
    curses.curs_set(0)          # Hide blinking cursor
    stdscr.nodelay(True)        # Non-blocking keyboard input
    stdscr.timeout(100)         # Game speed: 100ms per frame (10 FPS)

    height, width = stdscr.getmaxyx()
    # Keep playable box safely inside terminal boundaries
    box_h, box_w = min(20, height - 2), min(50, width - 2)

    snake = [[box_h // 2, box_w // 2 + i] for i in range(3)]
    direction = curses.KEY_LEFT
    food = [random.randint(2, box_h - 2), random.randint(2, box_w - 2)]
    score = 0

    opposites = {
        curses.KEY_UP: curses.KEY_DOWN,
        curses.KEY_DOWN: curses.KEY_UP,
        curses.KEY_LEFT: curses.KEY_RIGHT,
        curses.KEY_RIGHT: curses.KEY_LEFT,
    }

    while True:
        stdscr.clear()
        # Draw border walls
        for x in range(1, box_w):
            stdscr.addch(1, x, "#")
            stdscr.addch(box_h, x, "#")
        for y in range(1, box_h + 1):
            stdscr.addch(y, 1, "#")
            stdscr.addch(y, box_w, "#")

        stdscr.addstr(0, 2, f" DEBIAN 13 SNAKE | Score: {score} | Press 'q' to quit ")
        stdscr.addch(food[0], food[1], "*")

        for idx, (sy, sx) in enumerate(snake):
            stdscr.addch(sy, sx, "@" if idx == 0 else "o")

        stdscr.refresh()

        key = stdscr.getch()
        if key == ord("q"):
            break
        if key in opposites and key != opposites.get(direction):
            direction = key

        head_y, head_x = snake[0]
        if direction == curses.KEY_UP:
            head_y -= 1
        elif direction == curses.KEY_DOWN:
            head_y += 1
        elif direction == curses.KEY_LEFT:
            head_x -= 1
        elif direction == curses.KEY_RIGHT:
            head_x += 1

        new_head = [head_y, head_x]

        # Check wall or self collision
        if (
            head_y <= 1 or head_y >= box_h
            or head_x <= 1 or head_x >= box_w
            or new_head in snake
        ):
            stdscr.nodelay(False)
            stdscr.addstr(box_h // 2, box_w // 2 - 8, f" GAME OVER! Score: {score} ")
            stdscr.addstr(box_h // 2 + 1, box_w // 2 - 11, " Press any key to exit ")
            stdscr.getch()
            break

        snake.insert(0, new_head)

        if new_head == food:
            score += 10
            while food in snake:
                food = [random.randint(2, box_h - 2), random.randint(2, box_w - 2)]
        else:
            snake.pop()

if __name__ == "__main__":
    curses.wrapper(main)

