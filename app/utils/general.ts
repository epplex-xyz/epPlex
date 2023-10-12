
export function cloneObject(obj) {
    return JSON.parse(JSON.stringify(obj));
}

export function combineDateAndTime(date: Date, time: Date) {
    const year = date.getFullYear();
    const month = date.getMonth();
    const day = date.getDate();

    const hours = time.getHours();
    const minutes = time.getMinutes();
    const seconds = time.getSeconds();

    return new Date(year, month, day, hours, minutes, seconds);
}

export function validateTraits(input): boolean {
    // Validate if the parsed value is an array
    if (Array.isArray(input)) {
        // Check if each item in the array has the required properties
        const isValid = input.every((traitObject) =>
            Object.prototype.hasOwnProperty.call(traitObject, 'trait_type') &&
            Object.prototype.hasOwnProperty.call(traitObject, 'value')
        );

        if (!isValid) {
            return true;
        }
    }
    return false;
}


export function addExtension(originalString, newExtension) {
    const parts = originalString.split('.');

    if (parts.length >= 2) {
        // Remove the existing extension and add the new one
        parts.pop();
    }

    parts.push(newExtension);

    return parts.join('.');
}

export function spliceAddress(address: string) {
    return address.slice(0, 4) + '..' + address.slice(-4);
}
