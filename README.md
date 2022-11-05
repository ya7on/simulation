# simulation

Simple life simulation

## description

Creatures spawn randomly on canvas with 5% chance. Instead of brain, they have simple neural network that receives as input: id of their generation, remaining energy, direction, and if there is another creature nearby, other id and energy. Initially they have 10 energy points. Each turn, they can do one of following actions: move (spends 2 energy points), reproduct (spends 4 energy points and creates new creature in empty cell), attack (spends 1 energy point and kills chosen creature in cell near them), and rest / healing (spend 1 energy point). When energy runs out, creature dies. If creature is born: it has 10% chance to be born with full energy, otherwise it will have same amount of energy as parent. There is also 1% chance of "mutation", one of neurons in newborn creature will chang.

![image](https://user-images.githubusercontent.com/7967826/194777419-3d7e4794-9417-4377-8005-e8ef7f687b56.png)

## Legend

- Creature background color is generated randomly when mutated
- Creature number [0; 9] means generation number
- Green text color - reproduction
- Yellow text color - movement
- Red text color - attack
- White text color - healing

![image](https://user-images.githubusercontent.com/7967826/194778096-d6fcbae5-33fb-4501-9d86-725854a7e69e.png)
