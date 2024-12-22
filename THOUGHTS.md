

### main.rs to Structure
Entities consume components that apply systems, local systems shouldn't really know anything about their core component. 
`PlayerMovement` systems don't know `Player` actually exists, just that `PlayerMovement` exists and 
they can act on the entity containing that component (and unique global components like `Transform`).
If `PlayerMovement` needs to know something about `Player` or some sister system, probably use events. IDs are fine, too.

Doing this, Entities pull in Systems through Components. Feels reverse of Domain Models being pulled up out of DBs. 
Feels like data pulling in functionality, rather than functionality acting on data?


`entities/` and `systems/` feels really on the nose, but I'm gonna try it. 
Also feel like `systems/` should live in `entities/` since `entities/` pulls them up, but I'm just gonna put them next to each other for now.

I guess I'm describing data-driven design, I think. That'd be cool. Or I'm being naive about it.



### A Play
Systems create Entities... 
Young Simba: But, Dad, don't Components eat the Systems?
Mufasa: Yes, Simba, but let me explain. When Components die, our bodies become the Entities, and the Systems eat the Entities. 
And so we are all connected in the great Circle of Life.

I originally named my `TileAnimation` component `AnimateTile`, all my Components are nouns, Systems verbs? Nope, I named `PlayerMovement` system `player_movement`. 
`TileAnimation`'s main system is probably still going to be called `animate_tile`


Visibility seems like a unique global component like Transform, so I guess I can modify it pretty directly?


I just type these while I wait for it to compile.

Ooo, plugins made of entirely system/systems, `set_current_tile` being the first, defining an interaction between `Tile` and `Player`.

I think I have quite a lot of boilerplate, but it makes me feel organized a bit. Feel like if I knew more it could probably be cleaner.

Had `set_current_tile` controlling tile animation, going to have the plugin have an event listening system, and see if I can have current tile announce itself?

Maybe Player shouldn't be the only one who can set current tile and it should be all entities with a `TileSetter` component? 
I think for simplicity I'll make it just the player. I'd have to change the function to be more general and allow for multiple currents...
Also makes me curious if there'd be a better way to check than looping players and tiles.

I just remembered that the player triggering the tiles isn't a long term state... well, it will still initiate the scan, so it's kind of like triggering a tile, so same thing I suppose.


### Allow the Concept of the Tile
I think I'm going to bring `Tile` into a `TileAnimation` system and it feels like it goes against what I've typed above. I'm scared, but I'm gonna let it cook.
Maybe I'll go look at Events quick, but I feel like we're still going to need to know which Tile we're dealing with by checking current.

I could still try an event to trigger animate which will listen instead of polling the state of all tiles? But I don't think I'm getting the decoupling I want.
The event thing kind of feels like a lot of boilerplate right now...

My systems are communicating through Entity/Core Component properties (activated, I think so far only one system or orchestration is a writer). I wonder if that's a bad idea. Feel like doing these queries over and over may not be optimal either.
Ah, but shoot, what if multiple events activate a tile; who's in charge of determining neither event is happening?

It'd be amazing if I'm wrong and just doing everything perfectly reasonably.

> Also feel like systems should live in entities since entities pulls them up, but I'm just gonna put them next to each other for now.

Feel like this is pretty inaccurate now since I got these systems just pulling Entities willy nilly.

Well, I feel a bit spaghetti'd. Might have to think on this.

### Embrace the Events
I don't think everything is better, but this feels good for now and I'm going to have to rediscover some pain points.

If the setters just emit, then it happens once, and how I have it setup with the animation it runs once until cleared, so it'll clear on the next run. Might just work.

I'll turn the activated property into an activated event that has a payload of row/col, I suppose. `TileAnimation` already has to know about `Tile` so no loss there.

Very tempting to name these event handlers "consumer/producers", Kafka for life. 

IDs for my tiles beyond row col? I like that.

Got tile activation working through Events! Next we calculate some paths and figure out how to activate them on a delay.

Systems that perform actions like animation can consume events from systems that monitor for changes. 
Maybe some day the `PlayerMovement` system will take in `WindEvent`s that can be emitted by `WindSystem` and `MovementEvent`s from `PlayerInput`. 
Or maybe both can write a `MovementEvent`. Interesting how that event would probably live with `PlayerMovement` and not `Player`, like `TileActivated` does. 
Probably because there's only ever really one consumer, but I can see `TileActivated` having more than 1 consumer.
And if not, then we could probably have more concretely made it some sort of `RequestTileAnimationEvent` that lives in `TileAnimation`.

Should I be considering single-writer principle in this scenario, or intentionally breaking it? I already intentionally break it in real life...
If I were to follow single-writer, I'd need to make `TileAnimation` listen on multiple topics, but manage the same state. Best they're ordered and funneled into a single event, right?

If they performed completely different animations that wouldn't be a big deal I suppose 
(as long as we check no other animation is running for the tile, but even then they could just overlap provided they act on different properties).


### Time for Pathfinding
