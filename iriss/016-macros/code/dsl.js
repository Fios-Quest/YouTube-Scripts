let _js = () => {
    const heading = React.createElement(
        'h1',
        {className: 'example'},
        'Hello, world!'
    );

    return heading;
};


let _dsl = () => {
    const heading = (
        <h1 className="example">
            Hello, world!
        </h1>
    );

    return heading;
};