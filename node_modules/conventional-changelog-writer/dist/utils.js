const DATETIME_LENGTH = 10;
/**
 * Formats date to yyyy-mm-dd format.
 * @param date - Date string or Date object.
 * @returns Date string in yyyy-mm-dd format.
 */
export function formatDate(date) {
    return new Date(date).toISOString().slice(0, DATETIME_LENGTH);
}
/**
 * Safe JSON.stringify with circular reference support.
 * @param obj
 * @returns Stringified object with circular references.
 */
export function stringify(obj) {
    const stack = [];
    const keys = [];
    let thisPos;
    function cycleReplacer(value) {
        if (stack[0] === value) {
            return '[Circular ~]';
        }
        return `[Circular ~.${keys.slice(0, stack.indexOf(value)).join('.')}]`;
    }
    function serializer(key, value) {
        let resultValue = value;
        if (stack.length > 0) {
            thisPos = stack.indexOf(this);
            if (thisPos !== -1) {
                stack.splice(thisPos + 1);
                keys.splice(thisPos, Infinity, key);
            }
            else {
                stack.push(this);
                keys.push(key);
            }
            if (stack.includes(resultValue)) {
                resultValue = cycleReplacer(resultValue);
            }
        }
        else {
            stack.push(resultValue);
        }
        return resultValue;
    }
    return JSON.stringify(obj, serializer, '  ');
}
/**
 * Creates a compare function for sorting from object keys.
 * @param strings - String or array of strings of object keys to compare.
 * @returns Compare function.
 */
export function createComparator(strings) {
    if (typeof strings === 'string') {
        return (a, b) => (a[strings] || '').localeCompare(b[strings] || '');
    }
    if (Array.isArray(strings)) {
        return (a, b) => {
            let strA = '';
            let strB = '';
            for (const key of strings) {
                strA += a[key] || '';
                strB += b[key] || '';
            }
            return strA.localeCompare(strB);
        };
    }
    return strings;
}
//# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoidXRpbHMuanMiLCJzb3VyY2VSb290IjoiIiwic291cmNlcyI6WyIuLi9zcmMvdXRpbHMudHMiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IkFBS0EsTUFBTSxlQUFlLEdBQUcsRUFBRSxDQUFBO0FBRTFCOzs7O0dBSUc7QUFDSCxNQUFNLFVBQVUsVUFBVSxDQUN4QixJQUFtQjtJQUVuQixPQUFPLElBQUksSUFBSSxDQUFDLElBQUksQ0FBQyxDQUFDLFdBQVcsRUFBRSxDQUFDLEtBQUssQ0FBQyxDQUFDLEVBQUUsZUFBZSxDQUFDLENBQUE7QUFDL0QsQ0FBQztBQUVEOzs7O0dBSUc7QUFDSCxNQUFNLFVBQVUsU0FBUyxDQUFDLEdBQVk7SUFDcEMsTUFBTSxLQUFLLEdBQWMsRUFBRSxDQUFBO0lBQzNCLE1BQU0sSUFBSSxHQUFhLEVBQUUsQ0FBQTtJQUN6QixJQUFJLE9BQWUsQ0FBQTtJQUVuQixTQUFTLGFBQWEsQ0FBQyxLQUFjO1FBQ25DLElBQUksS0FBSyxDQUFDLENBQUMsQ0FBQyxLQUFLLEtBQUssRUFBRSxDQUFDO1lBQ3ZCLE9BQU8sY0FBYyxDQUFBO1FBQ3ZCLENBQUM7UUFFRCxPQUFPLGVBQWUsSUFBSSxDQUFDLEtBQUssQ0FBQyxDQUFDLEVBQUUsS0FBSyxDQUFDLE9BQU8sQ0FBQyxLQUFLLENBQUMsQ0FBQyxDQUFDLElBQUksQ0FBQyxHQUFHLENBQUMsR0FBRyxDQUFBO0lBQ3hFLENBQUM7SUFFRCxTQUFTLFVBQVUsQ0FBZ0IsR0FBVyxFQUFFLEtBQWM7UUFDNUQsSUFBSSxXQUFXLEdBQUcsS0FBSyxDQUFBO1FBRXZCLElBQUksS0FBSyxDQUFDLE1BQU0sR0FBRyxDQUFDLEVBQUUsQ0FBQztZQUNyQixPQUFPLEdBQUcsS0FBSyxDQUFDLE9BQU8sQ0FBQyxJQUFJLENBQUMsQ0FBQTtZQUU3QixJQUFJLE9BQU8sS0FBSyxDQUFDLENBQUMsRUFBRSxDQUFDO2dCQUNuQixLQUFLLENBQUMsTUFBTSxDQUFDLE9BQU8sR0FBRyxDQUFDLENBQUMsQ0FBQTtnQkFDekIsSUFBSSxDQUFDLE1BQU0sQ0FBQyxPQUFPLEVBQUUsUUFBUSxFQUFFLEdBQUcsQ0FBQyxDQUFBO1lBQ3JDLENBQUM7aUJBQU0sQ0FBQztnQkFDTixLQUFLLENBQUMsSUFBSSxDQUFDLElBQUksQ0FBQyxDQUFBO2dCQUNoQixJQUFJLENBQUMsSUFBSSxDQUFDLEdBQUcsQ0FBQyxDQUFBO1lBQ2hCLENBQUM7WUFFRCxJQUFJLEtBQUssQ0FBQyxRQUFRLENBQUMsV0FBVyxDQUFDLEVBQUUsQ0FBQztnQkFDaEMsV0FBVyxHQUFHLGFBQWEsQ0FBQyxXQUFXLENBQUMsQ0FBQTtZQUMxQyxDQUFDO1FBQ0gsQ0FBQzthQUFNLENBQUM7WUFDTixLQUFLLENBQUMsSUFBSSxDQUFDLFdBQVcsQ0FBQyxDQUFBO1FBQ3pCLENBQUM7UUFFRCxPQUFPLFdBQVcsQ0FBQTtJQUNwQixDQUFDO0lBRUQsT0FBTyxJQUFJLENBQUMsU0FBUyxDQUFDLEdBQUcsRUFBRSxVQUFVLEVBQUUsSUFBSSxDQUFDLENBQUE7QUFDOUMsQ0FBQztBQUVEOzs7O0dBSUc7QUFDSCxNQUFNLFVBQVUsZ0JBQWdCLENBRzlCLE9BQTRDO0lBQzVDLElBQUksT0FBTyxPQUFPLEtBQUssUUFBUSxFQUFFLENBQUM7UUFDaEMsT0FBTyxDQUFDLENBQUksRUFBRSxDQUFJLEVBQUUsRUFBRSxDQUFDLENBQUMsQ0FBQyxDQUFDLE9BQU8sQ0FBQyxJQUFJLEVBQUUsQ0FBQyxDQUFDLGFBQWEsQ0FBQyxDQUFDLENBQUMsT0FBTyxDQUFDLElBQUksRUFBRSxDQUFDLENBQUE7SUFDM0UsQ0FBQztJQUVELElBQUksS0FBSyxDQUFDLE9BQU8sQ0FBQyxPQUFPLENBQUMsRUFBRSxDQUFDO1FBQzNCLE9BQU8sQ0FBQyxDQUFJLEVBQUUsQ0FBSSxFQUFFLEVBQUU7WUFDcEIsSUFBSSxJQUFJLEdBQUcsRUFBRSxDQUFBO1lBQ2IsSUFBSSxJQUFJLEdBQUcsRUFBRSxDQUFBO1lBRWIsS0FBSyxNQUFNLEdBQUcsSUFBSSxPQUFPLEVBQUUsQ0FBQztnQkFDMUIsSUFBSSxJQUFJLENBQUMsQ0FBQyxHQUFHLENBQUMsSUFBSSxFQUFFLENBQUE7Z0JBQ3BCLElBQUksSUFBSSxDQUFDLENBQUMsR0FBRyxDQUFDLElBQUksRUFBRSxDQUFBO1lBQ3RCLENBQUM7WUFFRCxPQUFPLElBQUksQ0FBQyxhQUFhLENBQUMsSUFBSSxDQUFDLENBQUE7UUFDakMsQ0FBQyxDQUFBO0lBQ0gsQ0FBQztJQUVELE9BQU8sT0FBTyxDQUFBO0FBQ2hCLENBQUMifQ==