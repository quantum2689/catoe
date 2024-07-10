## catoe 

**catoe is an tic-tac-toe bot using the minmax algorthim to play**

## the interface 

How to Play Tic-Tac-Toe Against a Bot
Board Setup:

When you start the game, you'll see a 3x3 grid printed on the screen. Each cell in the grid represents a position where you or the computer can place either an "X" or an "O".
Game Start:

The game starts with an empty board. You are "X" and the computer is "O".
You'll be prompted to enter your move. The prompt will look something like this:
scss
Copy code
Enter your move (0-8):
The numbers from 0 to 8 correspond to the positions on the board, like this:
markdown
Copy code
0 | 1 | 2
---------
3 | 4 | 5
---------
6 | 7 | 8
Making Your Move:

Type the number corresponding to the position where you want to place your "X" and press Enter.
If the position is already taken or you enter an invalid number (not between 0 and 8), the game will ask you to try again.
Computer's Turn:

After you make your move, the computer will immediately make its move.
You'll see the board update with the computer's "O" in the chosen position.
Game Progress:

You and the computer will continue taking turns until one of you wins or the game ends in a draw.
The game checks after each move if there's a winner or if the game is a draw.
Winning the Game:

You win if you manage to get three of your "X" marks in a row, horizontally, vertically, or diagonally.
The computer wins if it gets three of its "O" marks in a row.
Ending the Game:

If a player wins, the game will display a message like "X wins!" or "O wins!".
If there are no more empty spaces on the board and no player has won, the game ends in a draw.
Restarting the Game:

After the game ends, you can choose to play again by restarting the program.
Example Interaction:
You start by making your move:

scss
Copy code
Enter your move (0-8): 4
You place an "X" at position 4.
The computer then makes its move:

mathematica
Copy code
. . . 
. X . 
. . . 
O . . 
. . . 
Enter your move (0-8):
The computer places an "O" on the board.
You continue taking turns until the game ends in a win or draw.
