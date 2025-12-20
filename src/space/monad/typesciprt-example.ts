function divide(x: number, y: number): number | null {
    if (y === 0) {
        return null;
    }
    return x / y;
}

function square(input: number): number {
    return input * input;
}

function to_string(input: number): string {
    return `${input}`;
}

const maybe_division = divide(4, 2);
// This won't work!
// const squared_division = square(maybe_division);

let maybe_division_squared = null;
if (maybe_division !== null) {
    maybe_division_squared = square(maybe_division);
}
let maybe_squared_string = null;
if (maybe_division_squared !== null) {
    maybe_squared_string = to_string(maybe_division_squared);
}

console.log(maybe_squared_string)

const if_not_null = <T, U>(input: T | null, f: (_: T) => U): U | null => {
    if (input !== null) {
        return f(input);
    }
    return null;
}

const maybe_division_squared_f = if_not_null(maybe_division, square);
const maybe_squared_string_f = if_not_null(maybe_division_squared_f, to_string);

console.log(maybe_squared_string_f);

// Inlined, yuck!
const maybe_inlined = if_not_null(if_not_null(maybe_division, square), to_string);
console.log(maybe_inlined);

// Composition
const create_if_not_null = <T, U>(f: (_: T) => U) => (input: T | null): U | null =>
    input !== null ? f(input) : null;

const if_not_null_square = create_if_not_null(square);
const if_not_null_to_string = create_if_not_null(to_string);
const maybe_composed = if_not_null_to_string(if_not_null_square(maybe_division));
console.log(maybe_composed);

console.log(maybe_squared_string == maybe_squared_string_f);
console.log(maybe_squared_string == maybe_inlined);
console.log(maybe_squared_string == maybe_composed);

