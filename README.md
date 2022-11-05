# simulation

Simple life simulation

## description

Creatures spawn randomly on canvas with 0.5% chance. Instead of brain, they have simple neural network that receives as input: id of their generation, remaining energy, direction, and if there is another creature nearby, other id and energy. Initially they have 10 energy points. Each turn, they can do one of following actions: move (spends 2 energy points), reproduct (spends 4 energy points and creates new creature in empty cell), attack (spends 1 energy point and kills chosen creature in cell near them), and rest / healing (spend 1 energy point). When energy runs out, creature dies. If creature is born: it has 10% chance to be born with full energy, otherwise it will have same amount of energy as parent. There is also 1% chance of "mutation", one of neurons in newborn creature will chang.

![image](https://user-images.githubusercontent.com/7967826/200130745-8d72d8c7-8097-47bb-aa69-f0bb9065d6c8.png)

## Legend

- Creature inner background color is generated randomly when mutated
- Green border color - reproduction
- Yellow border color - movement
- Red border color - attack
- White border color - healing

![image](https://user-images.githubusercontent.com/7967826/200130903-85561928-867f-4c6a-ad04-80d5bbbb08c5.png)
![image](https://user-images.githubusercontent.com/7967826/200130952-0e0705f0-1ba9-421f-9c18-54e494b5699e.png)
