# Known Bugs
Template
1. Short description / name
    * Reporter
    * Longer description
    * Recreation steps
      1. Step 1
      2. Step 2
      3. ...
    * Ideas or thoughts on fix

## Bugs
1. Can get stuck in wall
   * Reporter: Alvyn
   * See discord video
   * Make portals hit box in wall instead of out of wall

2. Teleporting above level
   * Reporter: Evan
   * Set down portals on first level and complete as usual. On second level, place one portal and try to enter it without placing the other
   * Do not allow teleport if portal isnt placed on current level (similar to if you dont place portal on first level and try to walk into it).

3. Momentum is conserved through levels
   * Reporter: Evan
   * Hit level clear hit box with a decent amount of speed. This could create unwanted actions performed in the next level.
   * Reset Player Velocity to Zero upon hitting level clear hit box.
