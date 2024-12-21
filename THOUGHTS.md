Entities consume components that apply systems, local systems shouldn't really know anything about their core component. 
PlayerMovement systems don't know Player actually exists, just that PlayerMovement exists and 
they can act on the entity containing that component (and unique global components like Transform).
If PlayerMovement needs to know something about Player or some sister system, probably use events.

Doing this, Entities pull in Systems through Components. Feels reverse of Domain Models being pulled up out of DBs. 
Feels like data pulling in functionality, rather than functionality acting on data?


`entities/` and `systems/` feels really on the nose, but I'm gonna try it. 
Also feel like systems should live in entities since entities pulls them up, but I'm just gonna put them next to each other for now.

I guess I'm describing data-driven design, I think. That'd be cool. Or I'm being naive about it.




Systems create Entities... 
Young Simba: But, Dad, don't Components eat the Systems?
Mufasa: Yes, Simba, but let me explain. When Components die, our bodies become the Entities, and the Systems eat the Entities. 
And so we are all connected in the great Circle of Life.
