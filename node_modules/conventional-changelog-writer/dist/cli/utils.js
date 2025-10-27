import { resolve, extname } from 'path';
import { pathToFileURL } from 'url';
import { readFile } from 'fs/promises';
const NEWLINE = /\r?\n/;
export async function* parseJsonStream(stream) {
    let chunk;
    let payload;
    let buffer = '';
    let json;
    for await (chunk of stream) {
        buffer += chunk.toString();
        if (NEWLINE.test(buffer)) {
            payload = buffer.split(NEWLINE);
            buffer = payload.pop() || '';
            for (json of payload) {
                try {
                    yield JSON.parse(json);
                }
                catch (err) {
                    throw new Error('Failed to split commits', {
                        cause: err
                    });
                }
            }
        }
    }
    if (buffer) {
        try {
            yield JSON.parse(buffer);
        }
        catch (err) {
            throw new Error('Failed to split commits', {
                cause: err
            });
        }
    }
}
export async function* readCommitsFromFiles(files) {
    for (const file of files) {
        try {
            yield JSON.parse(await readFile(file, 'utf8'));
        }
        catch (err) {
            console.warn(`Failed to read file ${file}:\n  ${err}`);
        }
    }
}
export function readCommitsFromStdin() {
    return parseJsonStream(process.stdin);
}
function relativeResolve(filePath) {
    return pathToFileURL(resolve(process.cwd(), filePath));
}
export async function loadDataFile(filePath) {
    const resolvedFilePath = relativeResolve(filePath);
    const ext = extname(resolvedFilePath.toString());
    if (ext === '.json') {
        return JSON.parse(await readFile(resolvedFilePath, 'utf8'));
    }
    // @ts-expect-error Dynamic import actually works with file URLs
    // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
    return (await import(resolvedFilePath)).default;
}
//# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoidXRpbHMuanMiLCJzb3VyY2VSb290IjoiIiwic291cmNlcyI6WyIuLi8uLi9zcmMvY2xpL3V0aWxzLnRzIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiJBQUNBLE9BQU8sRUFBRSxPQUFPLEVBQUUsT0FBTyxFQUFFLE1BQU0sTUFBTSxDQUFBO0FBQ3ZDLE9BQU8sRUFBRSxhQUFhLEVBQUUsTUFBTSxLQUFLLENBQUE7QUFDbkMsT0FBTyxFQUFFLFFBQVEsRUFBRSxNQUFNLGFBQWEsQ0FBQTtBQUV0QyxNQUFNLE9BQU8sR0FBRyxPQUFPLENBQUE7QUFFdkIsTUFBTSxDQUFDLEtBQUssU0FBUyxDQUFDLENBQUMsZUFBZSxDQUFJLE1BQWdCO0lBQ3hELElBQUksS0FBYSxDQUFBO0lBQ2pCLElBQUksT0FBaUIsQ0FBQTtJQUNyQixJQUFJLE1BQU0sR0FBRyxFQUFFLENBQUE7SUFDZixJQUFJLElBQVksQ0FBQTtJQUVoQixJQUFJLEtBQUssRUFBRSxLQUFLLElBQUksTUFBTSxFQUFFLENBQUM7UUFDM0IsTUFBTSxJQUFJLEtBQUssQ0FBQyxRQUFRLEVBQUUsQ0FBQTtRQUUxQixJQUFJLE9BQU8sQ0FBQyxJQUFJLENBQUMsTUFBTSxDQUFDLEVBQUUsQ0FBQztZQUN6QixPQUFPLEdBQUcsTUFBTSxDQUFDLEtBQUssQ0FBQyxPQUFPLENBQUMsQ0FBQTtZQUMvQixNQUFNLEdBQUcsT0FBTyxDQUFDLEdBQUcsRUFBRSxJQUFJLEVBQUUsQ0FBQTtZQUU1QixLQUFLLElBQUksSUFBSSxPQUFPLEVBQUUsQ0FBQztnQkFDckIsSUFBSSxDQUFDO29CQUNILE1BQU0sSUFBSSxDQUFDLEtBQUssQ0FBQyxJQUFJLENBQU0sQ0FBQTtnQkFDN0IsQ0FBQztnQkFBQyxPQUFPLEdBQUcsRUFBRSxDQUFDO29CQUNiLE1BQU0sSUFBSSxLQUFLLENBQUMseUJBQXlCLEVBQUU7d0JBQ3pDLEtBQUssRUFBRSxHQUFHO3FCQUNYLENBQUMsQ0FBQTtnQkFDSixDQUFDO1lBQ0gsQ0FBQztRQUNILENBQUM7SUFDSCxDQUFDO0lBRUQsSUFBSSxNQUFNLEVBQUUsQ0FBQztRQUNYLElBQUksQ0FBQztZQUNILE1BQU0sSUFBSSxDQUFDLEtBQUssQ0FBQyxNQUFNLENBQU0sQ0FBQTtRQUMvQixDQUFDO1FBQUMsT0FBTyxHQUFHLEVBQUUsQ0FBQztZQUNiLE1BQU0sSUFBSSxLQUFLLENBQUMseUJBQXlCLEVBQUU7Z0JBQ3pDLEtBQUssRUFBRSxHQUFHO2FBQ1gsQ0FBQyxDQUFBO1FBQ0osQ0FBQztJQUNILENBQUM7QUFDSCxDQUFDO0FBRUQsTUFBTSxDQUFDLEtBQUssU0FBUyxDQUFDLENBQUMsb0JBQW9CLENBQUksS0FBZTtJQUM1RCxLQUFLLE1BQU0sSUFBSSxJQUFJLEtBQUssRUFBRSxDQUFDO1FBQ3pCLElBQUksQ0FBQztZQUNILE1BQU0sSUFBSSxDQUFDLEtBQUssQ0FBQyxNQUFNLFFBQVEsQ0FBQyxJQUFJLEVBQUUsTUFBTSxDQUFDLENBQU0sQ0FBQTtRQUNyRCxDQUFDO1FBQUMsT0FBTyxHQUFHLEVBQUUsQ0FBQztZQUNiLE9BQU8sQ0FBQyxJQUFJLENBQUMsdUJBQXVCLElBQUksUUFBUSxHQUFhLEVBQUUsQ0FBQyxDQUFBO1FBQ2xFLENBQUM7SUFDSCxDQUFDO0FBQ0gsQ0FBQztBQUVELE1BQU0sVUFBVSxvQkFBb0I7SUFDbEMsT0FBTyxlQUFlLENBQUksT0FBTyxDQUFDLEtBQUssQ0FBQyxDQUFBO0FBQzFDLENBQUM7QUFFRCxTQUFTLGVBQWUsQ0FBQyxRQUFnQjtJQUN2QyxPQUFPLGFBQWEsQ0FBQyxPQUFPLENBQUMsT0FBTyxDQUFDLEdBQUcsRUFBRSxFQUFFLFFBQVEsQ0FBQyxDQUFDLENBQUE7QUFDeEQsQ0FBQztBQUVELE1BQU0sQ0FBQyxLQUFLLFVBQVUsWUFBWSxDQUFDLFFBQWdCO0lBQ2pELE1BQU0sZ0JBQWdCLEdBQUcsZUFBZSxDQUFDLFFBQVEsQ0FBQyxDQUFBO0lBQ2xELE1BQU0sR0FBRyxHQUFHLE9BQU8sQ0FBQyxnQkFBZ0IsQ0FBQyxRQUFRLEVBQUUsQ0FBQyxDQUFBO0lBRWhELElBQUksR0FBRyxLQUFLLE9BQU8sRUFBRSxDQUFDO1FBQ3BCLE9BQU8sSUFBSSxDQUFDLEtBQUssQ0FBQyxNQUFNLFFBQVEsQ0FBQyxnQkFBZ0IsRUFBRSxNQUFNLENBQUMsQ0FBVyxDQUFBO0lBQ3ZFLENBQUM7SUFFRCxnRUFBZ0U7SUFDaEUsc0VBQXNFO0lBQ3RFLE9BQU8sQ0FBQyxNQUFNLE1BQU0sQ0FBQyxnQkFBZ0IsQ0FBQyxDQUFDLENBQUMsT0FBaUIsQ0FBQTtBQUMzRCxDQUFDIn0=