# Meili x advent of code

--- Day Meili ---

Hey @scotow,

Your input look like that:
```
tamo - RLRLR
loic - RLLL
louis - LRL
kero - LRLR
luna - LRRR
lena - RLLR
thomas - LRLL
tommy - LLL
```

On the left is the name of the child that needs to get his gift.
On the right is the path to go to his house. Each `L` means santa needs to turn `left` and each `R` means he needs to turn `right`.
For example, for santa to go to luna's house he needs to turn `left`, `right`, `right`, `right`.

Since there is so many childs every year santa has a machine that create a structure which regroups all the childrens addresses with as little instructions as possible. [need a big rework] 
With the input above, this is what the structure would looks like:
```
           LL - - - - - - tommy
          /   L - - - - - thomas
         /   /
        /   L - R - - - - kero
       /   /
      L - R - RR - - - -  luna
     /    
    /      L - - - - - -  loic
- -       /
    \    L - R - - - - -  lena
     \  /
      RL - RLR - - - - -  tamo
         
```

This machine has a lot of issue, sometimes it takes days to answer and sometimes it output wrong results, thus santa has a second machine that ensure the result is correct.
In order to work, this second machine needs to get all the path sorted. The `left` turns first and then the `right` turns.

Your goal is to find which child is santa going to access with as little instructions as possible.
In this case `tommy` and `tamo` are the only two childs accessible in only two instructions. But since `tommy` will appear first in the structure he's going to get his gift first.


--- Part2 ---

For the second part you need to count how many steps it'll take to santa go from the closest to closest child (still in number of steps).
```
Step 0:
           LL - - - - - - tommy
          /   L - - - - - thomas
         /   /
        /   L - R - - - - kero
       /   /
      L - R - RR - - - -  luna
     /    
    /      L - - - - - -  loic
- -       /
    \    L - R - - - - -  lena
     \  /
      RL - RLR - - - - -  tamo
         
Step 1:
           LL - - - - - - 2 instructions
          /   L - - - - - thomas
         /   /
        /   L - R - - - - kero
       /   /
      L - R - RR - - - -  luna
     /    
    /      L - - - - - -  loic
- -       /
    \    L - R - - - - -  lena
     \  /
      RL - RLR - - - - -  tamo

Step 2:
              L - - - - - thomas
             /
            L - R - - - - kero
           /
      L - R - RR - - - -  5 instructions
     /    
    /      L - - - - - -  loic
- -       /
    \    L - R - - - - -  lena
     \  /
      RL - RLR - - - - -  tamo
      
Step 3:
              L - - - - - 9 instructions
             /
            L - R - - - - kero
           /
      L - R
     /    
    /      L - - - - - -  loic
- -       /
    \    L - R - - - - -  lena
     \  /
      RL - RLR - - - - -  tamo
      
Step 4:
            L - R - - - - 12 instructions
           /
      L - R
     /    
    /      L - - - - - -  loic
- -       /
    \    L - R - - - - -  lena
     \  /
      RL - RLR - - - - -  tamo
      
Step 5:
           L - - - - - -  loic
- -       /
    \    L - R - - - - -  lena
     \  /
      RL - RLR - - - - -  18 instructions

Step 6:
           L - - - - - -  22 instructions
- -       /
    \    L - R - - - - -  lena
     \  /
      RL
      
Step 7:
- -        
    \    L - R - - - - -  25 instructions
     \  /
      RL
```

Here, all gifts has been delivered in 2i instructions.
