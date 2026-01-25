class Maybe<T> {
    inner: NonNullable<T> | null;

    constructor(inner: NonNullable<T> | null) {
        this.inner = inner;
    }

    map<U>(f: (_: T) => NonNullable<U>): Maybe<U> {
        return this.inner === null
            ? new Maybe<U>(null)
            : new Maybe(f(this.inner));
    }
}

const maybe_squared_time_string = divide(4, 2)
    .map(square)
    .map(to_string);

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



console.log(maybe_squared_time_string);
