# Meili x advent of code

--- Day Meili ---

Hey @scotow,

Your input look like that:
```
tamo - RLRLR
loic - RLLL
kero - LRLR
luna - LRRR
lena - RLLR
thomas - LRLL
tommy - LLL
chayaline - LRLL
```

On the left is the name of the child that needs to get his gift.
On the right is the path to go to his house. Each `L` means Santa needs to turn `left`, and each `R` means he needs to turn `right`.
For example, for Santa to go to luna's house, he needs to turn `left`, `right`, `right`, and `right`.

Since there are so many children every year, Santa has a machine that creates a structure that regroups all the children's addresses with as few instructions as possible. [need a big rework] 
With the input above, this is what the structure would look like:
```
          LL - - - - - - tommy
         /   L - - - - - chayaline, thomas
        /   /
       /   L - R - - - - kero
      /   /
     L - R - RR - - - -  luna
    /    
   /      L - - - - - -  loic
- o      /
   \    L - R - - - - -  lena
    \  /
     RL - RLR - - - - -  tamo
         
```

This machine has a lot of issues, sometimes it takes days to answer, and sometimes it outputs wrong results. Thus Santa has a second machine that ensures the result is correct.
In order to work, this second machine needs to get all the paths sorted. The `left` turns first, and then the `right` turns.

Your goal is to find which child Santa can access with as few instructions as possible.
In this case, `Tommy` and `tamo` are the only two children accessible in only two instructions. But since `Tommy` will appear first in the structure, he's going to get his gift first.


--- Part2 ---

For the second part, you need to count how many instructions it'll take to Santa go from the closest to the closest child (still in a number of instructions).
```
Step 0: We're at the beginning
          LL - - - - - - tommy
         /   L - - - - - chayaline, thomas
        /   /
       /   L - R - - - - kero
      /   /
     L - R - RR - - - -  luna
    /    
   /      L - - - - - -  loic
- o      /
   \    L - R - - - - -  lena
    \  /
     RL - RLR - - - - -  tamo
         
Step 1: tommy was the closest, 2 instructions away.
          LL - - - - - - 2 instructions
         /   L - - - - - chayaline, thomas
        /   /
       /   L - R - - - - kero
      /   /
     L - R - RR - - - -  luna
    /    
   /      L - - - - - -  loic
- o      /
   \    L - R - - - - -  lena
    \  /
     RL - RLR - - - - -  tamo

Step 2: luna was the closest, 3 instructions away.
             L - - - - - chayaline, thomas
            /
           L - R - - - - kero
          /
     L - R - RR - - - -  5 instructions
    /    
   /      L - - - - - -  loic
- o      /
   \    L - R - - - - -  lena
    \  /
     RL - RLR - - - - -  tamo
      
Step 3: chayaline and thomas were the closest, 3 instructions away.
             L - - - - - 8 instructions
            /
           L - R - - - - kero
          /
     L - R
    /    
   /      L - - - - - -  loic
- o      /
   \    L - R - - - - -  lena
    \  /
     RL - RLR - - - - -  tamo
      
Step 4: kero was the closest, 2 instructions away.
           L - R - - - - 10 instructions
          /
     L - R
    /    
   /      L - - - - - -  loic
- o      /
   \    L - R - - - - -  lena
    \  /
     RL - RLR - - - - -  tamo
      
Step 5: tamo was the closest, 6 instructions away
          L - - - - - -  loic
- o      /
   \    L - R - - - - -  lena
    \  /
     RL - RLR - - - - -  16 instructions

Step 6: loic was the closest, 3 instructions away.
          L - - - - - -  19 instructions
- o      /
   \    L - R - - - - -  lena
    \  /
     RL
      
Step 7: lena was the closest, 2 instructions away.
- o        
   \    L - R - - - - -  21 instructions
    \  /
     RL
```

Here, all gifts have been delivered in 25 instructions.
