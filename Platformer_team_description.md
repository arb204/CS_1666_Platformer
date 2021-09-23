# CS 1666 - Platformer  

[Link to our repo](https://github.com/arb204/CS_1666_Platformer)

## Our Team:  
**Networking Advanced Topic**  
    Andrew Ashley (Pitt ID: ata33, Github: andrew-ashley10)
    Alvyn Berg (Pitt ID: apb62, Github: Alvynskio)
    Austin Brothers (Pitt ID: arb204, Github: arb204)
    Evan Moran (Pitt ID: ecm61, Github: Evmo98)
    Greg Sack (Pitt ID: grs76, Github: gregdrew12)
**Rigidbody Physics Advanced Topic**  
    Bryce Anderson (Pitt ID: baa104, Github: baa104)
    Joshua Mateer (Pitt ID: jcm155, Github: jcm155)
    Jake Sweeney (Pitt ID: jms608, Github: jsweeney12)
    Kira Tsvetkova (Pitt ID: mat253, Github: kiraghost)

## Game Description:  
We are creating a puzzle platformer inspired by the __Portal__ series of games. The gameplay will involve placing portals to traverse obstacles that aren't traversible via the normal platforming methods (running, jumping, dashing, etc.) The goal of each stage is simply to reach the exit gate/door, typically located at the right side of the screen.

__Concept Art__  
__Level Concept__\
![alt text](https://github.com/andrew-ashley10/CS_1666_Platformer/blob/main/2021-09-10_13.37.14.png "Level 1")\
__Main Character Animation__\
![alt text](https://github.com/andrew-ashley10/CS_1666_Platformer/blob/main/gif-character-orange.gif "Character 1")\
__Portal Animation__\
![alt text](https://github.com/andrew-ashley10/CS_1666_Platformer/blob/main/gif-portal-blue.gif "Portal Animation")\
__Concept Additional Character Animation__\
![alt text](https://github.com/andrew-ashley10/CS_1666_Platformer/blob/main/walking.gif "Character 2")\


## Advanced Topics  
**Networking**  
Networking means communicating with other systems or machines to use inputs from each machine to influence this system. We will use this advanced topic to develop a multiplayer mode where each player will control a single portal and each player will have to work together to reach the goal.

**Physics**
Physics is an essential feature in all platformer-esque games, as collision detection and rigidbodies are essential to traverse a level and create a world where you can run, jump, and interact with different objects. We will implement this into the game by creating a rigidbody system that allows for rectangular and ellipsoid colliders that are affected by gravity and can collide with other rigidbodys. We will use this to create player characters, levels that these characters can explore, and simple objects to comeplete puzzles in each stage.

## Specific Milestones  
**Gameplay Mechanics**  
* Basic character movement (running, jumping)  
* Rigidbody physics system (colliding, gravity, rolling)  
* Online mutliplayer  
* Portal creation  
* Moving characters through portals with momentum conserved  
* Moving objects through portals with momentum conserved  

**Scope**  
The scope of the game will be 5 levels, each with a static camera where a single level takes up the entirety of the screen.
* __Stretch Goal 1: Curved Portal Creation__
    We want to create items that allow the user to throw a potion that creates a portal where it lands. This would allow users to get more creative with how they want to explore levels.
* __Stretch Goal 2: Movement Abilities__
    We want to allow the user to have more movement abilities, such as a horizontal midair dash or a double jump. This would allow users to explore levels with more freedom and perform more actions after traveling through a portal.

**Advanced Topics Milestones**
* Networking
    By the end of the semester, we want to have a fairly low-latency (<200ms) online multiplayer mode where both players can create and travel through portals in real time, where players do not appear to "teleport" (game lagging and correcting player positions). Players should be able to see the other player at all times and disconnects due to server errors should almost never happen.
* Physics  
    By the end of the semester, we want to have a robust physics system that allows for both rectangular and ellipsoid objects. We want to be simulating all physics for these rigidbodies, including weight, gravity, collisions, and friction. We also want these calaculations to not slow down the gamepay in any noticeable way regardless of how many objects we have on screen (within reason of course)

**Midterm Milestones**
* Basic graphics (character models and level assets)  
* Basic rigidbody physics sytem (rectangular colliders that have gravity/weight/friction)  
* Basic portal creation/traversal (objects can teleport but do not conserve momentum)  
* One basic "test" level to show off mechanics  
* Progress towards online functionality but most likely will not be shown for midterm evaluation  

**Final Milestones**
__MVP (75%)__
    * 5 unique levels/puzzles (15%)  
    * Online multiplayer where each player controls one character and can create one portal (20%)  
    * Physics system that supports both rectangular and ellipsoid colliders and physics (20%)  
    * Portal creation system that allows two portals to be created on any valid surface, allows objects and characters to travel through them while conserving momentum (20%)  
__Reaching MVP (15%)__
__Stretch Goal 1: Curved Portal Creation__
__Stretch Goal 2: Movement Abilities (5%)__
