class Maybe<T> {
    inner: NonNullable<T> | null;

    constructor(inner: NonNullable<T> | null) {
        this.inner = inner;
    }

    map<U>(f: (_: T) => U): Maybe<U> {
        if (this.inner !== null) {
            return new Maybe(f(this.inner) || null);
        }
        return new Maybe<U>(null);
    }
}

function divide(x: number, y: number): Maybe<number> {
    if (y === 0) {
        return new Maybe<number>(null);
    }
    return new Maybe(x / y);
}

function square(input: number): number {
    return input * input;
}

function to_string(input: number): string {
    return `${input}`;
}

const maybe_squared_time_string = divide(4, 2).map(square).map(to_string);

console.log(maybe_squared_time_string);
