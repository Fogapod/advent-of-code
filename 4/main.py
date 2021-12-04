with open("input") as f:
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
def board_wins(board) -> bool:
    for i in range(board_side):
        rows_marked = 0
        cols_marked = 0

        for j in range(board_side):
            rows_marked += int(board[i][j][1])
            cols_marked += int(board[j][i][1])

        if board_side in (rows_marked, cols_marked):
            return True

    return False

def mark_number(board, number: int):
    for row in board:
        for cell in row:
            if cell[0] == number:
                cell[1] = True

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
            mark_number(board, number)

            if board_wins(board):
                print(sum_unmarked(board) * number)
                return


winning_board()

# task 2
def last_winning_board():
    remaining_boards = boards.copy()

    for number in numbers:
        did_not_win = []

        for board in remaining_boards:
            mark_number(board, number)

            if board_wins(board):
                if len(remaining_boards) == 1:
                    print(sum_unmarked(board) * number)
                    return
            else:
                did_not_win.append(board)

        remaining_boards = did_not_win

last_winning_board()
