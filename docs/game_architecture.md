# Architecture

An overview of what components (and sub-components) make up Space Station 13
for the purpose of allowing versatile modding without sacrificing literally
all structure and by extension, performance.


## Users

Users can be either players or spectators, and any user can be assigned
an admin role.

### Players

Players make up the majority of a server's users, and can perform
normal actions, which include changing servers as well as all of the actions
that make up gameplay. They each have an avatar and a perspective tied to
that avatar, even in death.

They each have an inventory (including items and currency) and a set of actions
they can take based on their class.

### Spectators

Spectators are able to see everything happening in a game, including who is an
antagonist, but are not visible to the players and cannot chat with them. They
are able to chat with other spectators, and can be allowed to join as players by
petitioning the server's admins.

### Admin Privileges

When a user is given admin privileges, they are able to control which mod pack
is loaded for the server, as well as manage the players, including whitelisting
or blacklisting them.


## Tiles

Tiles are the square units that make up each map. They can either be floors or
walls, which are different in how other entities interact/collide with them, but
they work very similarly.

### Floors

Floor tiles are tiles with no collision of their own that render a square image
or animation by default. They also have a stack of zero or more effect layer or
objects on top of them, that each render on top of the tile and can apply effects
upwards, either to static objects or entities on top of the given layer.

### Walls

Wall tiles are built on top of floor tiles with dynamic or static collision. This
includes either normal walls that always block players from standing on them, as
well as doors that are sometimes passable. The difference between walls and objects
sitting in place on a floor tile is that walls obscure the view of players.
