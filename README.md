# Meili x advent of code

--- Day Meili ---

🎅 Santa needs your help once again to save this festive moment 🎄

With the world population officially hitting 8 billion people, Santa's job isn't getting easier. Enter the ✨Gift-o-tron-3000✨
Santa could already see it! Using all the saved time to enjoy more cookies and a few extra glasses of milk left by the kids! The perfect Christmas all around...
Or so it should have been.

Unfortunately, the shady elf merchant of this machine "forgot" to mention that the **Gift Positioning System** (GPS) of the machine significantly slows down with the number of kids increasing.
Luckily, we can provide our own GPS algorithm to the Gift-o-tron-3000! Lucky indeed, you know your way around code. They don't call you the sharpest coder of the North Pole for nothing!

All the kids of the world and Santa themself are counting on you! Let operation SOS GPS begin ✨

The kids' address list looks as follows:
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

On the left is the kid's name that needs to get their gift.
On the right is the path from Santa's home to the kid's house. An `L` means Santa needs to turn `left`, and an `R` means they need to turn `right`.
For example, for Santa to go to luna's house, they need to turn `left`, `right`, `right`, and `right`.

Unfortunately (again), Rudolph has a bad left foot. We need to prioritize turning left before they run out of steam.

With the input above, our new GPS map must look like this:
```
          LL - - - - - - tommy
         /   L - - - - - chayaline, thomas
        /   /
       /   L - R - - - - kero
      /   /
     L - R - RR - - - -  luna
    /    
   /      L - - - - - -  loic
  o      /
   \    L - R - - - - -  lena
    \  /
     RL - RLR - - - - -  tamo
         
```

The reindeer will make a stop every time there is an intersection.
Your goal is to find which kid Santa can get to with as few instructions as possible and gives his name.

In the above case, `tommy` and `tamo` are the only kids accessible in two instructions. But since `tommy` will appear first in the structure, he gets his gift first.


--- Part2 ---

You are able to find the closest kids! But what about the others? The old GPS became so slow that we would not have been able to deliver all the gifts on time!
Now, you need to count how many stops it'll take Santa to go from the closest to the next closest kid until there are no kids remaining.
Even kids in Antarctica must receive their well deserved gifts!

**You need to find how many stops Santa will make.**
The number of stops will help Santa know how many carrots they needs to bring for their dear reindeer.

```
Step 0: We're at Santa's house
          LL - - - - - - tommy
         /   L - - - - - chayaline, thomas
        /   /
       /   L - R - - - - kero
      /   /
     L - R - RR - - - -  luna
    /    
   /      L - - - - - -  loic
  o      /
   \    L - R - - - - -  lena
    \  /
     RL - RLR - - - - -  tamo
         
Step 1: tommy is the closest, 2 stops away.
          LL - - - - - - 2 stops in total
         /   L - - - - - chayaline, thomas
        /   /
       /   L - R - - - - kero
      /   /
     L - R - RR - - - -  luna
    /    
   /      L - - - - - -  loic
  o      /
   \    L - R - - - - -  lena
    \  /
     RL - RLR - - - - -  tamo

Step 2: luna is the closest, 3 stops away.
             L - - - - - chayaline, thomas
            /
           L - R - - - - kero
          /
     L - R - RR - - - -  5 stops in total
    /    
   /      L - - - - - -  loic
  o      /
   \    L - R - - - - -  lena
    \  /
     RL - RLR - - - - -  tamo
      
Step 3: chayaline and thomas are the closest, 3 stops away.
             L - - - - - 8 stops in total
            /
           L - R - - - - kero
          /
     L - R
    /    
   /      L - - - - - -  loic
  o      /
   \    L - R - - - - -  lena
    \  /
     RL - RLR - - - - -  tamo
      
Step 4: kero is the closest, 2 stops away.
           L - R - - - - 10 stops in total
          /
     L - R
    /    
   /      L - - - - - -  loic
  o      /
   \    L - R - - - - -  lena
    \  /
     RL - RLR - - - - -  tamo
      
Step 5: tamo is the closest, 6 stops away
          L - - - - - -  loic
  o      /
   \    L - R - - - - -  lena
    \  /
     RL - RLR - - - - -  16 stops in total

Step 6: loic is the closest, 3 stops away.
          L - - - - - -  19 stops in total
  o      /
   \    L - R - - - - -  lena
    \  /
     RL
      
Step 7: lena is the closest, 2 stops away.
  o        
   \    L - R - - - - -  21 stops in total
    \  /
     RL
```

In the example above, we have 21 stops.
Don't forget: If two kids are at the same distance, you need to deliver your gift to the kid that's more to the left of Santa's house.
