A friend and I raced to complete the same sudoku puzzle, with drinking forfeits for each row, column, and 3x3 sub-grid. While the drinking may be a hard sell for television, they'll be begging me for a framework for evaluating which of the next deducible cells will unlock the most insight into the sudoku along with a deduction difficulty rating, allowing for real time commentary on how much 2 or more competitors are deviating from the optimal line and how impressive it is when they find a very hard cell. 

Front end will be written in C# because I've never used it, back end written in Rust because I'm cute like that.

Roadmap: 
- Move solvers (Naked Singles, Hidden Singles) and board candidate evaluators (Pointed Pairs/Triples + Quads and above, Box-Line Reduction, X-Wing / Swordfish)
- Automatic board solver with move logic output (looping above)
- Insight2difficulty function ("What is the easiest move leading to the most amount of easy discoveries?")
- C# gui to let you play with or without squares highlighted for either required depth search or insight2difficulty score 

Cool to have:
- optimal path finder function (ordered list of moves which minimises high depth searches) + active path deviation score
- timer and other cool gui features 
- library of puzzles and/or puzzle generation code

To do:
- finish depth 2 evaluation function
- include depth rating in move struct + change single move finders to include assigning evaluation depth variable 
- rating function -> f(# of discovered moves, evaluation cost of initial and discovered moves): gives all moves a an insight2difficulty rating and applies it to the associated cells of the board, updated every applied for gameplay gui purposes (can have 2 colour coded cell options: 1. depth of search, 2. difficulty rating )
