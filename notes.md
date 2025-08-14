# Notes

## To DO
- [?] Move everything to UUID
- [x] Move as much code out of code gen
- [x] Conversations
	- [ ] Conversation Triggers
- [ ] In game editor
	- [ ] Editor actions as subset of Game Actions
	- [ ] In build, double array lengths to allow editing in app (in edit mode)
- [x] State changes and saves
- [ ] Design interface
- [ ] Using Items
- [ ] Quests
- [ ] Fonts

## General

- You are in a location, what's there?
	- Exits -> Items with movement triggers
	- Items
	- People -> trigger Combat, Conversation, Trade, etc
	- Metadata?

[Entity Component System](https://en.wikipedia.org/wiki/Entity_component_system)
https://www.codingwiththomas.com/blog/an-entity-component-system-from-scratch
```
Location: Tavern Main Room
Entities:
	BarTender:
		Talk -> Trigger Conversation XYZ
		Barter -> Trigger Market XYZ
	Jerk:
		Talk -> Trigger -> Converstaion ABC
		Attack -> Trigger -> Fight with Jerk
	Candle:
		(Take) Move to Me -> Trigger -> Move Candle
		Use with <Fire> -> Trigger -> Swap Lit Candle
Components:
	Exit to ABC
	Exit to DEF
	Description
	LongDescription
```

To do all this, you need:
- A means of intellegently reading/searching/aggregating/understanding a complete, _machine-readable_ data set.
	- ie jq in a fancy outfit
	- logseq, knowledge graph
- Means to check for dangling data points (unreachable dialog, undeclared state)
- Converting data to code
- Converting data to state
- ID duplication

Naming (IDing) things will be hard, so use metadata/metatags/metadescription to find and link
Conditional DSL
How to declare state? - Start 0 unless declared?
I think YML might be the best bet unless there is a good way to write hierarchy in TOML
- Node anchors
https://www.npmjs.com/package/yaml-schema-validator
https://github.com/crdoconnor/strictyaml


- Create a data structure that can be used by any type of system.

WORLD MAP locations (other than special locs) ie travel mid points / random encounters are made on the fly??

World Map:
 - Is it a map? Can I travel to random locations? Or is it lines from city to city?


Persisting state changes...
Start with base world state
