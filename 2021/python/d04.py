from pathlib import Path


file = Path(__file__)

with open(file.parent.parent / "input" / file.stem / "full") as f:
    numbers = [int(n) for n in f.readline().split(",")]
    f.readline()

    board_lines = f.read().split("\n\n")

# board cell contents:
# [number: int, is_marked: bool]

boards = []
for board in board_lines:
    rows = []
    for line in board.rstrip().split("\n"):
        rows.append([[int(n), False] for n in line.split()])

    boards.append(rows)

board_side = len(boards[0])

# task 1
def mark_number(board, number: int) -> bool:
    for i in range(board_side):
        for j in range(board_side):
            if board[i][j][0] != number:
                continue

            board[i][j][1] = True

            # assume numbers are unique on board

            score_x = 0
            score_y = 0

            for k in range(board_side):
                score_x += int(board[i][k][1])
                score_y += int(board[k][j][1])

            if board_side in (score_x, score_y):
                return True

            break

    return False


def sum_unmarked(board) -> int:
    total = 0

    for row in board:
        for cell in row:
            if not cell[1]:
                total += cell[0]

    return total


def winning_board():
    for number in numbers:
        for board in boards:
            if mark_number(board, number):
                print(sum_unmarked(board) * number)
                return


winning_board()

# task 2
def last_winning_board():
    remaining_boards = boards.copy()

    for number in numbers:
        did_not_win = []

        for board in remaining_boards:
            if mark_number(board, number):
                if len(remaining_boards) == 1:
                    print(sum_unmarked(board) * number)
                    return
            else:
                did_not_win.append(board)

        remaining_boards = did_not_win


last_winning_board()
