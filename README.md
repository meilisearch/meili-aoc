# Meili x advent of code

--- Day Meili ---

Hey @scotow,

🎅 Santa needs your help once again to save this festive moment 🎄

Since there are so many kids every year, Santa needed a machine that creates a route to go from santa's house to any kids adresse in an efficient way. To do so, he bought the Gift-o-tron-3000 machine. Santa could already see it! With all that time saved, they'll be able to enjoy many more cookies left by the kids, and finish a glass of milk or two as well! The perfect Christmas all around... Or so it should have been.

Unfortunately, the mean, shady and sly elf merchant of this machine, "forgot" to mention that the **Gift Positioning System** (GPS) of the machine  significantly slows down with the number of kids increasing.

Luckily, we can provide our own GPS algorithm in the Gift-o-tron-3000. Lucky indeed, you know your way around a code. They don't call you the sharpest coder of the North Pole for nothing!

All the kids of the world and Santa themself are counting on you! Let operation SOS GPS begin ✨

The kids adress list looks like that:
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

On the left is the name of the kid that needs to get their gift.
On the right is the path to go from Santa's home to the kids house. A `L` means Santa needs to turn `left`, and a `R` means they needs to turn `right`.
For example, for Santa to go to luna's house, they need to turn `left`, `right`, `right`, and `right`.

Unfortunately (again), Rudolph has a bad left foot, we need to prioritize turning left before they run out of steam.

With the input above, our new GPS map must looks like this:
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

As a first step, your goal is to find which kid Santa can access with as few instructions as possible.

In the above case, `Tommy` and `tamo` are the only two kids accessible in only two instructions. But since `Tommy` will appear first in the structure, he's going to get his gift first.


--- Part2 ---

You are able to find the closests kids! But what about the others? The old GPS system became so slow that we would not have been able to deliver all the gifts on time!

For the second part, you need to count how many stops it'll take to Santa to go from the closest to the next closest kid until there is no kids remaining.

Even kids in Antartica must recieve their well deserved gifts!

The number of stops helps us knowing how many carrots Santa must bring along to fuel the reins. For every stop, all of the 4 reins needs 1 carrot, but rudolph need 3 because he is a star ✨ and because apparently "his left foot hurts".

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
  o      /
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
  o      /
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
  o      /
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
  o      /
   \    L - R - - - - -  lena
    \  /
     RL - RLR - - - - -  tamo
      
Step 4: kero was the closest, 2 instructions away.
           L - R - - - - 10 instructions
          /
     L - R
    /    
   /      L - - - - - -  loic
  o      /
   \    L - R - - - - -  lena
    \  /
     RL - RLR - - - - -  tamo
      
Step 5: tamo was the closest, 6 instructions away
          L - - - - - -  loic
  o      /
   \    L - R - - - - -  lena
    \  /
     RL - RLR - - - - -  16 instructions

Step 6: loic was the closest, 3 instructions away.
          L - - - - - -  19 instructions
  o      /
   \    L - R - - - - -  lena
    \  /
     RL
      
Step 7: lena was the closest, 2 instructions away.
  o        
   \    L - R - - - - -  21 instructions
    \  /
     RL
```

In the example above, we have 21 stops, which means 6 carrots times 21.

The final solution would be 126 carrots.

