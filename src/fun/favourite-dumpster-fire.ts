"use strict"

// ---

const gary = Array(16).join('wat' - 1) + ' Batman!';
console.log(gary);

// ---

type Dog = {
    name: string,
}

const dog1: Dog = { name: 'Fido' };
const dog2 = Object.create(dog1);
const dog3 = Object.create(dog2);

const dogs = [dog1, dog2, dog3];

const printDogName = (dog: Dog) => console.log(dog.name);

for (const dog of dogs) {
    printDogName(dog)
}

// ---

dog1.name = 'Rover';
dog2.name = 'Lassie';

for (const dog of dogs) {
    console.log(dog.name)
}

for (const dog of dogs) {
    console.log(dog.hasOwnProperty('name'))
}


interface Cat {
    age: number;
}

interface Cat {
    name: string;
}

const cat1 = Object.create(null);
cat1.name = 'Yuki';
cat1.age = 17;

const describeCat = (cat: Cat) => {
    console.log(`${cat.name} is ${cat.age} years old`);
    // cat.hasOwnProperty('age');
    console.log(Object.prototype.hasOwnProperty.call(cat, 'age'));
    console.log('age' in cat);
}

describeCat(cat1);
describeCat(dog1 as Cat);
printDogName(cat1);

const isDog = (maybeDog: unknown): maybeDog is Dog => (
    typeof maybeDog === 'object'
    && maybeDog !== null
    && 'name' in maybeDog
);