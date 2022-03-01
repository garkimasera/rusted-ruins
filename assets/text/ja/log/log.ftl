
# Message at start

start = Rusted Ruins へようこそ! (version : {$version})

# Messages about debug command

debug-command-invalid = Invalid debug command.
debug-command-need-1arg = Debug command "{$command}" needs 1 argument.
debug-command-failed = Debug command "{$command}" failed.
debug-command-genchara = Character "{$chara}" is generated.
debug-command-genitem = Item "{$item}" is generated.

# Messages about tile information

tile-information-no-info = このタイルには特に情報が無い。

# Messages when moving on map

exit-to-outside = {$player}は外へ出た。
enter-site = {$player}は{$site}へ入った。
change-floor = {$player}は隣のフロアへ向かった。

# Messages about character status

level-up = {$chara}のレベルが{$level}に上がった。
skill-level-up = {$chara}の{$skill}レベルが上がった。
skill-learned = {$chara}は{$skill}スキルを習得した。
skill-already-learned = {$chara}はすでに{$skill}スキルを覚えている。

# Messages about npc ai

npc-get-hostile = {$chara}は敵対した。

# Messages about combat

attack = {$attacker}は{$target}を攻撃した。
shot-target = {$attacker}は{$target}を撃った。
no-ranged-weapon-equipped = 遠隔武器を装備していない。
no-target = {$chara}はターゲットを見つけられなかった。
target-chara = {$chara}は{$target}をターゲットにした。
attack-evade = {$chara}は攻撃を避けた。
damaged-chara = {$chara}はダメージを受けた({$damage})。
arrow-hit = 矢は{$chara}に命中した。
throw-item = {$chara}は{$item}を投げた。
killed = {$chara}は倒された。
killed-by-melee-attack = {$chara}は倒された。
killed-by-ranged-attack = {$chara}は倒された。
killed-by-explosion = {$chara}は爆死した。
killed-by-poison-damage = {$chara}は毒により死んだ。
killed-by-starve-damage = {$chara}は餓死した。

# Messages about character action

item-container-capacity-limit = それを入れる容量がない。
item-equip = {$chara}は{$item}を装備した。
item-pickup = {$chara}は{$item}を拾った。
item-pick-up-plant = {$item}は植物であり、拾うことはできない。
item-pick-up-fixed = {$item}は固定されている。
item-drop = {$chara}は{$item}を床に置いた。
item-owned-by-others = {$item}は他者の所有物である。
drink-item = {$chara}は{$item}を飲み干した。
eat-item = {$chara}は{$item}を食べた。
harvest-plant = {$chara}は{$item} x {$n}を収穫した。
harvest-plant-not-ready = {$item}はまだ収穫できないようだ。
use-ability-ether = {$chara}は"{$ability}"を使用した。
use-ability-special = {$chara}は"{$ability}"を使用した。
ability-not-enough-mp = {$chara}のMPが足りない。
ability-not-enough-sp = {$chara}のSPが足りない。

# Messages about using tools

use-tool-without-equip = 道具を持っていない！
building-not-adjacent-tile = 建築には隣接タイルを指定しなければならない。
building-shortage-material = 建築には{$item} x {$n}が必要だ。
chopping-no-tree = 切り倒す木が無い。
chopping-not-adjacent-tile = 木を切り倒すには隣接タイルを指定しなければならない。
mining-not-adjacent-tile = 掘るには隣接タイルを指定しなければならない。

# Messages about using items

inventory-item-rotten = 荷物の中の{$item} x {$n}が腐敗した。
use_item-deed-invalid-map = ここで権利証を使うことはできない。
use_item-deed-occupied = 権利証を使うには、占拠されていない土地が必要だ。
use_item-deed-succeed = 新たな家を建設した！

# Messages when a character is affected

heal-hp = {$chara}は回復した({$value})。
fall-asleep = {$chara}は眠りに落ちた。
poisoned = {$chara}は毒を受けた。
scanned = {$chara}のスキャンが完了した。
not-scanned = {$chara}はまだスキャンされていない。
asleep = {$chara}は眠っている。
poison-damage = {$chara}は毒のダメージを受けた({$damage})。

# Messages about shops

shop-lack-of-money = {$chara}はそれを買うのに十分なお金を持っていない。
install-slot-lack-of-money = {$chara}はスロットの取付けに必要なお金を持っていない。

# Messages about quest

quest-report-completed-quests = {$player}は達成したクエストを報告した。

# Messages about factions
faction-relation-improve = {$faction}との関係が改善した ({$value})。
faction-relation-lower = {$faction}との関係が悪化した (-{$value})。

# Messages about creation

creation-start = {$chara}は{$product}の作成を始めた。
creation-finish = {$chara}は{$product}の作成を終了した。

# Messages about container
container-convert-item = {$item} x {$n}は{$container}により変換された。

# Messages about harvest

harvest-chop = {$chara}は伐採により{$item} x {$n}を手に入れた。
harvest-deconstruct = {$chara}は分解した素材から{$item} x {$n}を回収した。
harvest-plant = {$chara}は{$item} x {$n}を収穫した。

# Messages about party
party-add-chara = {$chara}があなたに付いてくことになった！

# Messages about script

player-receive-item = {$chara}は{$item} x {$n}を受け取った。
player-receive-money = {$chara}は{$amount}シルバーを受け取った。
