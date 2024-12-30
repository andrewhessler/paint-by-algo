

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


### Thank You 
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
Should I:
1. Produce a path calculation from a path calculating system to be consumed by `TileAnimation` and maybe other Services.
1. Have a System that both caculates the path and is then in charge of activating tiles with `TileActivated` events.

I'm not liking the `TileActivated` Writer setup. I think I want `emit_current_tile_as_activated` to just emit `CurrentTile` events.

`Tile` can consume `CurrentTile` and emit `TileActivated`, which `TileAnimation` consumes. 
For pathfinding we'll probably emit `TileVisited` and `TileChecked`, and `TileAnimation` will consume those as well.
This is going to get crazy, I feel like I'm going to need some sort of debouncing.

I don't feel this is getting too convoluted, though `TileAnimation` could just consume `CurrentTile`... let's do that for now, 
though I'm not saying there's anything wrong with funneling into `TileActivated` if it had significant meaning in context.

For now, I think I want things to react specifically to `CurrentTile`, and the calculation service probably will, too. 
Every plugin is becoming its own little bundle of microservices.

Also going back to single-writer again I think, so maybe it was right all along, 
though still think a single plugin writing in multiple places counts as a single writer, like microservices, also still the inverse for notification systems? 
Unless you do tailored notifications, and that announces things it observes about topics? 
Seems like too much logic, could have some sort of observer service for unique scenarios, but probably always an easy async way to send notifications with multi-writer to the queue.

It's weird that animating the current tile depends on `EmitCurrentTilePlugin` being included... 
but just because it's decoupled doesn't mean it isn't dependent for functionality. Think replication. I think it's nice, maybe.

Should all the emitters go into their own emitter folder in systems? Maybe some day.

For now `emit_pathfinding` will handle actually calculating the path and controlling the rate of emitting the events `TileChecked` and `TileVisited`.

Should something like `CurrentTile` be on a timer or at least deduping? 
We'll see if it causes issues. For now we have some local deduping in like `TileAnimation`, but should probably store previous_current locally and dedupe.

Okay, deduping `CurrentTileEvent`s done, I swear I'll actually do the pathfinding now. 
I'm going to let each algorithm read a collection of `Tile`s directly and spit out visit/checked with id.

After I'm done doing this all with one animation, need to clean up animation so it's easier for one animation to lock the tile until the animation is complete.

I hate this.
Maybe Tiles should live in a Grid.

Should animations stay queued or be dropped?

Everything in pathfinding event emitting feels wrong. I don't think I'll have a full picture of what I should do until I get back to TileAnimation. 
I'll make this silly way work for now if I can.

Going to naively wait to consume pathfinding events in `AnimateTile`. Probably have some issues with making the ripple smooth.


PATHFINDING PRETTY MUCH ANIMATING!

### Quick Check-in
Think there's something wrong with my algorithm implementation. But could be wrong. 
I really want to do what I can to clean up the animation, I want a new queue of ripples to fire off on every new current tile. 
Can I just assume the entire event read is an entire calculation? I could maybe make it that way, but I'll assume for now to see what happens.

Could think about pathfinding emitting a `CalculationOrderEvent` or something that contains all events instead of separating out the individual pathfinding events...

Really quick, long term want to add placing walls, water, moving the end, adding wind that moves the player. More algorithms. Probably UI and configuration.

### Algorithm and Animation Clean Up
Put my Deques in a Vec and now I got ripples. There's probably a way better way to do this stuff. Gonna play with a second animation and look at the algorithm accuracy.
Also may check how other people are doing their animations.

Took ages to learn how to change the color of a tile. Need to read up on assets and handles more. 
Seems like if components are associated with an Asset, they old the handle to that Asset and you query for the handle.


### Quicker Algorithm
Rignt now I batch tile animations. I'm thinking about grabbing every checked leading up to a visited and batching that way instead of by count. 
But I also wouldn't mind just a less noisy trail. Maybe see if there's something I did wrong with my algorithm

I'm trying just emitting Visited... not sure what others did, still gotta go look at that repo.

Okay, doing only `Visited` with exactly the color effect I want. The structure of Animation needs a lot of clarifying, that's probably next. 
Also some day probably a config option for blipping `Checked`.

### I wanna add walls
I really want people to be able to place Walls, but I think it will make developing that feature much more exciting if Walls already work.
Oh, but first I should fix animation/everything structure.

Starting with separating PlayerInput and PlayerMovement.

I separated them out, but have butchered movement controls with how I consume them. 
Probably track all 4 directions individually and evaluate them.

Okay, so now my movement is smoother and you can't hit a combination of keys where you won't be moving even though you're holding down a key. 
BUT, there's a W and A bias, instead of a last pressed bias.

Okay, I got it so that "last press wins" and also "living presses continue", so holding A and then D, go D, then let go of D, start going A.
Pretty sure the old way had the W/A bias as well. So this is an improvement, just not sure if there's a more readable way to do it.

### Animation structure thoughts
Animation has specific events it consumes for and then runs the corresponding animation, so on the `CurrentTileEvent`, we run an animation... hmm, kind of already doing that.
I might not mind the current structure. It's a reasonable amount of specific. I'll at least move it out into `animation/`.

I guess next is a `Wall`.


### Time for `Wall`
I think I'm going to just make it an attribute and then in the future when I decide them at runtime I can have a listener that decorates them appropriately? 
Also probably move End to being part of this `TileType` enum.

Oh shoot, gonna need collision. Well, that'll probably take some time.

I think I'll do something naive, like if you enter a `Wall` tile, you get knocked back. `CurrentTile` can include the tile type now and `PlayerMovement` can consume it.

Okay, so create a `Collidable` component; on new `CurrentTileEvent` check if inside `Collidable`. 
Emit `CollidedEvent` that specifies the angle from the origin the `Player` is to the `Collidable`.
`PlayerMovement` consumes `CollidedEvent` and pushes the player back.

I think I like the rebounding, seems pretty decent to me, I think this way will result in some weird fun bounce houses.

Next will be world wrapping and I guess `End` tile resetting the game.

Maybe I'll make `WorldWrapping` super generic and it just checks everything with a `Transform`, that sounds like a bad idea...
Maybe only the `Player` can wrap for now.

Okay, world wrapping going great, a few hiccups with the interpolation movement, but got it figured out mostly.

Next, make the end tile end game. Hmmm, this seems hard. Oh god, I might not do it. We'll save that for much later.
Though, I might be upset about not handling it for everything right now? Oh well, shouldn't be too hard since it'll still be pretty isolated to the entities.

### Pathfinding World Wrap
Next, I want to see if I can make my pathfinding world wrap... I think I can do that if I just duplicate the grid on all 4 sides? Or do I have to do it on 9 sides?

I should maybe start doing some profiling before I implement this.

Oh! Nevermind, I can just mod the index access by the row/col count.
Well, after a bit of a silly struggle with converting a negative `isize` to a `usize`, World Wrap Pathfinding is working! Some day I'll make it configurable.

I think I'm going to make it so you can trigger a calculation with a button press, instead of on current tile. 
Time to go write a new event in `PlayerInput` and a new consumer over in `pathfinding/`.

Ope, nevermind, just need a new consumer, `PlayerInput` pretty generous with the keys.

It feels like cheating that all the updates are essentially synchronous. I'm probably doing this wrong, but feels like it's guaranteed events will be consumed in order.

```rust
fn trigger_pathfinding_by_button(
    mut player_input_reader: EventReader<PlayerInput>,
    mut current_tile_reader: EventReader<CurrentTileEvent>,
) {
    for event in current_tile_reader.read() {
        // ...
    }
    for input in player_input_reader.read() {
        // ...
    }
}
```

Alright, anyway. That's working great! Can now use `J` to pulse the pathfinding.

Guess I should make some sort of starting UI to show the controls.

### Opening Screen
Feel like it's going to be a pain to make the world wait to spawn, still not sure the best way to go about that. Will probably relate to resetting.

### Actually, Second Pathfinding Algorithm, A*?
Probably A* because it's basically Dijkstra and I don't know any others yet.

Added A*, tried to multiply the distance by a constant to hopefully punish distance more harshly. 

There's still an issue where it can't see the end through the world wrap unless it naturally checks the edge. Then if the world wrap side is faster that will win.
I think I can fix this by calculating distance to imaginary reflected end tiles. 

This is actually apparently called "toirodal wrapping" and there are some ways to [calculate it](https://stackoverflow.com/a/3041398).

And just grabbing from that answer, I drew it out to make sense of it. Since we're working in the context of a bounded plane, 
if the distance on an axis (dx, dy) is larger than half the max value of the axis, then it must be more efficient to wrap. 
Conveniently, the distance it must be if you wrap is the `max axis value` - `the unwrapped distance`.

But something is still wrong with my A*, I don't think it's making the correct shape... Or maybe it is, world wrapping is just funny?


### Change Algorithm on demand, maybe UI, maybe rework pathfinding to send events in a bundle
Reworked pathfinding to send in a bundle, sort of. Should also change animation to using an enum.

### No UI for now, boo UI, change algorithm on demand and add/remove walls
For algorithm change on demand, listen for 1 or 2 input in `pathfinding/` I guess?

Added an aggressive A* that cares way more about direction. I think it's all working with the world wrap, but sometimes I'm skeptical.

Next walls, probably going to want to emit a `CurrentMouseTile`, animation to outline it in white. 
Why not make a `wallbuilding/` module that will consume `CurrentMouseTile` and `PlayerInput`. 
On left click, set current tile to Wall, on right click set current tile to OPEN.
Then we'll have different build states, Wall/End, maybe look at adding multiple ends/no ends. 
I'm gonna wanna add the ability to change individual tile colors some day...


Okay, everything is doing everything, I have made monolith spaghetti. `wallbuilding/` is just weird. 
I should really do some diagramming, but at this point I just want to add things so oh well. The number of events is getting out of hand though.

Ope nevermind, back to events lol

Should I just be passing entity handles around? Does that mean if I delete something I could be acting on nothing? 
But probably not because it uses some sort of reference counting?

Left Click done, time for right click and then I can clean up the random wall.

Right click probably good enough.

We'll do moving the `End` next. And after that probably small guy that runs to the end. Probably change `wallbuilding/` to `tile_modifier/`
Use `E` and `R` to change between `BuildingState`, `End` and `Wall`, emit `TerrainEvent`s instead of `WallEvent`s.

`Tile` is in charge of modifying attributes, `TileAnimation` is in charge of modifying visuals -- loosely...

Okay, it's basically all following the rules, but following the rules feels like microservices. Anyway, moving `End` working, can also remove the `End`.
Now to remove the default `Wall`s!

Okay, moved the end to a corner, my A* world wrapping is definitely not working.


### Next time, debug A*!
