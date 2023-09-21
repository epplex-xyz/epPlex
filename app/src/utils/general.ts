
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
