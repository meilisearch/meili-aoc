# Meili x advent of code

--- Day Meili ---

Hey @scotow,

🎅 Santa needs your help once again to save this festive moment 🎄

Since there are so many children every year, Santa needed a machine that creates a structure that regroups all the children's addresses with as few instructions as possible. 

Santa has invested a lot in a machine called the Gift-o-tron-3000 to facilitate his gift-giving tour. 

Unfortunately, the last update for the **Gift Positioning System** (GPS) feature has deteriorated its functioning. 

As the CTO of Santa Inc, you are the best elf to help Santa on this critical issue since everyone else is already on vacation!

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

With the increasing number of children, this year the GPS feature became unresponsive. 
Luckily we can provide our own algorithm to the Gift-o-tron-3000.
But our algorithm must output its structure in the exact same order that the previous algorithm was outputting.
While building this tree, all the paths must be sorted, the `left` turns first, and then the `right` turns.

Your goal is to find which child Santa can access with as few instructions as possible.

In this case, `Tommy` and `tamo` are the only two children accessible in only two instructions. But since `Tommy` will appear first in the structure, he's going to get his gift first.


--- Part2 ---

For the second part, you have a brilliant improvement in mind. You're going to count how many instructions it'll take to Santa to go from the closest to the next closest child until there is no child left.
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
