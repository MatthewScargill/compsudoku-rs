A friend and I raced to complete the same sudoku puzzle, with drinking forfeits for each row, column, and 3x3 sub-grid. While the drinking may be a hard sell for television, they'll be begging me for a framework for evaluating which of the next deducible cells will unlock the most insight into the sudoku along with a deduction difficulty rating, allowing for real time commentary on how much 2 or more competitors are deviating from the optimal line and how impressive it is when they find a very hard cell. 

Written in Rust because I'm cute like that.

Roadmap: 
- Board evaluator (Naked Singles, Hidden Singles, Naked Pairs, Box-Line Reduction, Hidden Pairs, X-Wing / Swordfish)
- Board solver (just looping the above)
- Insight function 
- find balanced "worth it for the difficulty" score using ease and insight
- gui which lets you play sudoku while highlighting the possible moves (colour coded for score)
- optimal play score 
- timer and other stuff 
- test and enjoy with library of possible puzzles (maybe even randomised puzzle generator)