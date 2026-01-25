[...document.getElementsByTagName('input')]
    .filter(element => element.type === 'checkbox')
    .forEach(element => element.disabled = false);
