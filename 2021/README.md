# How to Run
The package should be first built with 'cargo build' or 'cargo run' and then the executable can be run independently. 

# My Attempts with Comments

I think it's most efficient to make one cargo project with one main file, and then run each day (from another imported .rs file) from there. 

## Day 0
Set up calling functions from other files. Using a "match" expression for flexibility.

## Day 1
Calculating relative depths. First for adjacent measurements and then for a sliding window 3 measurements wide. Used Vectors and a for-loop, so I'm sure it could be more efficient. 

## Day 2
Calculate the changing position of our submarine. The commands come in form ("command", value) and needed to be parsed. Pretty straightforward use of a match expression once I split the commands and position into structs.

## Day 3
Needed to calculate the most and least common bit values in series of bit arrays. For part 1 it was easy to take the sum of an array of "bytes" along each column, and then check whether the sum was > len(array)/2, if so set the value to 1 for gamma, 0 otherwise. Epsilon was always !gamma. In part 2, the summation had to be redone for each set of "valid" bit arrays, but otherwise same logic. I could probably have made it more efficient in part 2 only taking sum of an individual column rather than all bits, but addition is cheap...

## Day 4
Bingo! How hard could it be... I stored each "board" as a hashmap (like python dict) for fast lookup of whether a number was in a board. This worked well for part 1, finding the first winning board. Part 2 is finding the last, and for whatever reason it works on the test data but not the real input yet.

## Day 5
Given line start/end coordinates, find where they cross. I made Line and Map structs to hold the data, and saved the coordinates along each line. I found out that you can't make an array [usize;1000000] but you can make a vector equally long, due to stack overflow. Part 1 was just horiz/vert lines, and part 2 added diagonal. All the magic happens in the file processing function which formats the lines nicely.

## Day 6
Exponential fish spawning! In part 1 I simply made a Fish struct and iterated overall fish on a given day, making more fish when timers ran out. This of course becomes crazy for a large number of days, and part 2 punished me for taking the easy way out. In part 2 I have an array with N_fish on each day, and then I spawn fish in batches and "school" the fish at the end of a cycle to account for new spawns. Should scale linearly with N_days. 

## Day 7
A minimization problem. I thought I could use the median of the array (which worked for test data), and this works for part 1. For part two I used the mean because now outliers were more heavily weighted. 

## Day 8
Seven Segment Search -- needed to parse strings like 'gedfc' to figure out which digit is being displayed. The trick is that we don't know what char corresponds to which display element! Part one was straightforward, count all strings of len 2,3,4,7 because they are unique. The strings with len 5 and 6 can represent 3 numbers each. Part 2 requires some logical deduction. I used hashmaps to store the code, number pairs and went to town. Needed to sort the input strings because the input strings were given with different character ordering that the output strings for each number to decrypt. I'm sure there's some better heuristics than what I used.

## Day 9
Smoky Snorkeling -- finding relative minima. Part 1 was straightforward because it didn't count diagonals. Part 2 involved finding "basins" which were clusters of points around the minima which have values < 9. I did this iteratively, adding unique points to a hashmap and then looking at their adjacent values.

## Day 10
Fun with stacks -- keeping track of opening and closing brackets. I tried to do it with keeping running total for each char (e.g., '['=1 and ']'=-1 ) but this was order agnostic and wasn't suited for either part. In the end I used stacks to track the ordering and pop or push as necessary at each character. Part 2 is then trivial as you then just keep popping and inverting the characters to get the "autocomplete". Reused the sorted insert function from previous days.

## Day 11
Recursion! -- when a squid flashes, recursively change adj squids' energy levels. Trick was to use hashmap so each  was only visited once.

## Day 12
Using depth-first search find paths through caves with only visiting small caves once. In part 2 you can visit up to one small cave 2x (this was implemented with a hashmap and simple iteration overall nodes -- probably a bit slow.). Part 1 is fast, but the iterations in part 2 are inefficient in my implementation.

## Day 13
Lots of x,y coordinate transforms to get to the final answer. Used Structs and hashmaps to only keep unique "dots"

## Day 14
I started the completely wrong way. I wasted a lot of time thinking I could do regex replacement (in the regex crate there is no way to do overlapping replacements that I could find), so then I started modifying the string directly. This worked for part one but part two took forever to complete. Completely rewrote using simple counting of pair and character occurances, and adding or subtracting the number of times they appear (based on James' soln in Julia).

## Day 15
This day was fun, I spent a lot of time reading about and implementing A* pathfinding, and it worked relatively well.

## Day 16
