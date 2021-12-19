
# Message at start

start = Welcome to Rusted Ruins! (version : {$version})

# Messages about debug command

debug-command-invalid = Invalid debug command.
debug-command-need-1arg = Debug command "{$command}" needs 1 argument.
debug-command-failed = Debug command "{$command}" failed.
debug-command-genchara = Character "{$chara}" is generated.
debug-command-genitem = Item "{$item}" is generated.

# Messages about tile information

tile-information-no-info = No information about this tile.

# Messages when moving on map

exit-to-outside = {$player} exited to outside.
enter-site = {$player} entered {$site}.
change-floor = {$player} moved to the next floor.

# Messages about character status

level-up = {$chara}'s level reached {$level}.
skill-level-up = {$chara}'s {$skill} level increased.
skill-learned = {$chara} learned {$skill} skill.
skill-already-learned = {$chara} have already learned {$skill} skill.

# Messages about npc ai

npc-get-hostile = {$chara} turned against you.

# Messages about combat

attack = {$attacker} attacked {$target}.
shot-target = {$attacker} shot {$target}.
no-ranged-weapon-equipped = No ranged weapon equipped!
no-target = {$chara} could not find any target.
target-chara = {$chara} targeted {$target}.
attack-evade = {$chara} evaded.
damaged-chara = {$chara} was damaged ({$damage}).
arrow-hit = The arrow hit {$chara}.
throw-item = {$chara} threw one {$item}.
killed-by-melee-attack = {$chara} was killed.
killed-by-ranged-attack = {$chara} was killed.
killed-by-explosion = {$chara} was killed.
killed-by-poison-damage = {$chara} was killed by poison.
killed-by-starve-damage = {$chara} starved to death.

# Messages about character action

item-container-capacity-limit = There is no space for it.
item-equip = {$chara} equipped {$item}.
item-pickup = {$chara} picked up {$item}.
item-pick-up-plant = {$item} is a living plant and cannot be moved.
item-pick-up-fixed = {$item} is fixed and cannot be moved.
item-drop = {$chara} dropped {$item}.
item-owned-by-others = {$item} is owned by others.
drink-item = {$chara} drank a {$item}.
eat-item = {$chara} ate a {$item}.
harvest-plant = {$chara} harvested {$item} x {$n}.
harvest-plant-not-ready = {$item} cannot be harvested yet.
use-ability-magic = {$chara} used magic "{$ability}".
use-ability-special = {$chara} used special skill "{$ability}".

# Messages about using tools

use-tool-without-equip = No tool equiped!
building-not-adjacent-tile = Need to specify an adjacent tile to build.
building-shortage-material = Need {$item} x {$n} more to build it.
chopping-no-tree = There is no tree to chop.
chopping-not-adjacent-tile = Need to specify an adjacent tile to chop.
mining-not-adjacent-tile = Need to specify an adjacent tile to mine.

# Messages about using items
inventory-item-rotten = {$item} x {$n} in your inventory were rotten.
use_item-deed-invalid-map = You can not use deeds in this area.
use_item-deed-occupied = You can not use deeds at this occupied area.
use_item-deed-succeed = You built new home!

# Messages when a character is affected

heal-hp = {$chara} was healed ({$value}).
fall-asleep = {$chara} fell asleep.
poisoned = {$chara} was poisoned.
scanned = {$chara} was scanned.
not-scanned = {$chara} isn't scanned.
asleep = {$chara} is asleep.
poison-damage = {$chara} was damaged by poison ({$damage}).
shop-lack-of-money = {$chara} do not have enough money to buy it.

# Messages about quest

quest-complete-slay_monsters = Quest completed! Slayed {$monster} x {$n}.
quest-reward-receive-money = Recieved {$money} gold as the quest reward.

# Messages about factions
faction-relation-improve = Relations with {$faction} improved by {$value}.
faction-relation-lower = Relations with {$faction} lowered by {$value}.

# Messages about creation

creation-start = {$chara} started making {$product}.
creation-finish = {$chara} finished making {$product}.
recipe-learned = {$chara} learned a new recipe "{$item}".
recipe-learning-failed = {$chara} could not learn a new recipe from this item.

# Messages about container
container-convert-item = {$item} x {$n} was converted by {$container}.

# Messages about harvest

harvest-chop = {$chara} chopped a tree, and got {$item} x {$n}.
harvest-deconstruct = {$chara} got {$item} x {$n} from deconstructed materials.
harvest-plant = {$chara} harvested {$item} x {$n}.

# Messages about party
party-add-chara = {$chara} attended your party.

# Messages about script

player-receive-item = {$chara} received {$item} x {$n}.
player-receive-money = {$chara} received {$amount} gold.
