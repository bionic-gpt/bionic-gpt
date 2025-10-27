function preventModifications(object) {
    return new Proxy(object, {
        get(target, prop) {
            const value = target[prop];
            // https://github.com/conventional-changelog/conventional-changelog/pull/1285
            if (value instanceof Date) {
                return value;
            }
            if (typeof value === 'object' && value !== null) {
                return preventModifications(value);
            }
            return value;
        },
        set() {
            throw new Error('Cannot modify immutable object.');
        },
        deleteProperty() {
            throw new Error('Cannot modify immutable object.');
        }
    });
}
/**
 * Apply transformation to commit.
 * @param commit
 * @param transform
 * @param args - Additional arguments for transformation function.
 * @returns Transformed commit.
 */
export async function transformCommit(commit, transform, ...args) {
    if (typeof transform === 'function') {
        const patch = await transform(preventModifications(commit), ...args);
        if (patch) {
            return {
                ...commit,
                ...patch,
                raw: commit
            };
        }
        return null;
    }
    return commit;
}
//# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiY29tbWl0LmpzIiwic291cmNlUm9vdCI6IiIsInNvdXJjZXMiOlsiLi4vc3JjL2NvbW1pdC50cyJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiQUFLQSxTQUFTLG9CQUFvQixDQUFzQixNQUFTO0lBQzFELE9BQU8sSUFBSSxLQUFLLENBQUMsTUFBTSxFQUFFO1FBQ3ZCLEdBQUcsQ0FBQyxNQUFNLEVBQUUsSUFBWTtZQUN0QixNQUFNLEtBQUssR0FBRyxNQUFNLENBQUMsSUFBSSxDQUFZLENBQUE7WUFFckMsNkVBQTZFO1lBQzdFLElBQUksS0FBSyxZQUFZLElBQUksRUFBRSxDQUFDO2dCQUMxQixPQUFPLEtBQUssQ0FBQTtZQUNkLENBQUM7WUFFRCxJQUFJLE9BQU8sS0FBSyxLQUFLLFFBQVEsSUFBSSxLQUFLLEtBQUssSUFBSSxFQUFFLENBQUM7Z0JBQ2hELE9BQU8sb0JBQW9CLENBQUMsS0FBSyxDQUFDLENBQUE7WUFDcEMsQ0FBQztZQUVELE9BQU8sS0FBSyxDQUFBO1FBQ2QsQ0FBQztRQUNELEdBQUc7WUFDRCxNQUFNLElBQUksS0FBSyxDQUFDLGlDQUFpQyxDQUFDLENBQUE7UUFDcEQsQ0FBQztRQUNELGNBQWM7WUFDWixNQUFNLElBQUksS0FBSyxDQUFDLGlDQUFpQyxDQUFDLENBQUE7UUFDcEQsQ0FBQztLQUNGLENBQUMsQ0FBQTtBQUNKLENBQUM7QUFFRDs7Ozs7O0dBTUc7QUFDSCxNQUFNLENBQUMsS0FBSyxVQUFVLGVBQWUsQ0FDbkMsTUFBYyxFQUNkLFNBQTJILEVBQzNILEdBQUcsSUFBVTtJQUViLElBQUksT0FBTyxTQUFTLEtBQUssVUFBVSxFQUFFLENBQUM7UUFDcEMsTUFBTSxLQUFLLEdBQUcsTUFBTSxTQUFTLENBQUMsb0JBQW9CLENBQUMsTUFBTSxDQUFDLEVBQUUsR0FBRyxJQUFJLENBQUMsQ0FBQTtRQUVwRSxJQUFJLEtBQUssRUFBRSxDQUFDO1lBQ1YsT0FBTztnQkFDTCxHQUFHLE1BQU07Z0JBQ1QsR0FBRyxLQUFLO2dCQUNSLEdBQUcsRUFBRSxNQUFNO2FBQ1osQ0FBQTtRQUNILENBQUM7UUFFRCxPQUFPLElBQUksQ0FBQTtJQUNiLENBQUM7SUFFRCxPQUFPLE1BQU0sQ0FBQTtBQUNmLENBQUMifQ==