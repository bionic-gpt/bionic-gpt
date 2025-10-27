import { Transform } from 'stream';
import { loadTemplates, createTemplateRenderer } from './template.js';
import { getFinalContext } from './context.js';
import { getFinalOptions, getGenerateOnFunction } from './options.js';
import { transformCommit } from './commit.js';
async function getRequirements(context = {}, options = {}) {
    const templates = await loadTemplates(options);
    const finalOptions = getFinalOptions(options, templates);
    const finalContext = getFinalContext(context, finalOptions);
    const generateOn = getGenerateOnFunction(finalContext, finalOptions);
    const renderTemplate = createTemplateRenderer(finalContext, finalOptions);
    return {
        finalContext,
        finalOptions,
        generateOn,
        renderTemplate
    };
}
export function writeChangelog(context = {}, options = {}, includeDetails = false) {
    const requirementsPromise = getRequirements(context, options);
    const prepResult = includeDetails
        ? (log, keyCommit) => ({
            log,
            keyCommit
        })
        : (log) => log;
    return async function* write(commits) {
        const { finalContext, finalOptions, generateOn, renderTemplate } = await requirementsPromise;
        const { transform, reverse, doFlush, skip } = finalOptions;
        let chunk;
        let commit;
        let keyCommit;
        let commitsGroup = [];
        let neverGenerated = true;
        let result;
        let savedKeyCommit = null;
        let firstRelease = true;
        for await (chunk of commits) {
            commit = await transformCommit(chunk, transform, finalContext, finalOptions);
            keyCommit = commit || chunk;
            if (skip?.(keyCommit)) {
                continue;
            }
            // previous blocks of logs
            if (reverse) {
                if (commit) {
                    commitsGroup.push(commit);
                }
                if (generateOn(keyCommit, commitsGroup)) {
                    neverGenerated = false;
                    result = await renderTemplate(commitsGroup, keyCommit);
                    commitsGroup = [];
                    yield prepResult(result, keyCommit);
                }
            }
            else {
                if (generateOn(keyCommit, commitsGroup)) {
                    neverGenerated = false;
                    result = await renderTemplate(commitsGroup, savedKeyCommit);
                    commitsGroup = [];
                    if (!firstRelease || doFlush) {
                        yield prepResult(result, savedKeyCommit);
                    }
                    firstRelease = false;
                    savedKeyCommit = keyCommit;
                }
                if (commit) {
                    commitsGroup.push(commit);
                }
            }
        }
        if (!doFlush && (reverse || neverGenerated)) {
            return;
        }
        result = await renderTemplate(commitsGroup, savedKeyCommit);
        yield prepResult(result, savedKeyCommit);
    };
}
/**
 * Creates a transform stream which takes commits and outputs changelog entries.
 * @param context - Context for changelog template.
 * @param options - Options for changelog template.
 * @param includeDetails - Whether to emit details object instead of changelog entry.
 * @returns Transform stream which takes commits and outputs changelog entries.
 */
export function writeChangelogStream(context, options, includeDetails = false) {
    return Transform.from(writeChangelog(context, options, includeDetails));
}
/**
 * Create a changelog string from commits.
 * @param commits - Commits to generate changelog from.
 * @param context - Context for changelog template.
 * @param options - Options for changelog template.
 * @returns Changelog string.
 */
export async function writeChangelogString(commits, context, options) {
    const changelogAsyncIterable = writeChangelog(context, options)(commits);
    let changelog = '';
    let chunk;
    for await (chunk of changelogAsyncIterable) {
        changelog += chunk;
    }
    return changelog;
}
//# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoid3JpdGVycy5qcyIsInNvdXJjZVJvb3QiOiIiLCJzb3VyY2VzIjpbIi4uL3NyYy93cml0ZXJzLnRzIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiJBQUFBLE9BQU8sRUFBRSxTQUFTLEVBQUUsTUFBTSxRQUFRLENBQUE7QUFRbEMsT0FBTyxFQUNMLGFBQWEsRUFDYixzQkFBc0IsRUFDdkIsTUFBTSxlQUFlLENBQUE7QUFDdEIsT0FBTyxFQUFFLGVBQWUsRUFBRSxNQUFNLGNBQWMsQ0FBQTtBQUM5QyxPQUFPLEVBQ0wsZUFBZSxFQUNmLHFCQUFxQixFQUN0QixNQUFNLGNBQWMsQ0FBQTtBQUNyQixPQUFPLEVBQUUsZUFBZSxFQUFFLE1BQU0sYUFBYSxDQUFBO0FBRTdDLEtBQUssVUFBVSxlQUFlLENBRzVCLFVBQTJCLEVBQUUsRUFDN0IsVUFBMkIsRUFBRTtJQUU3QixNQUFNLFNBQVMsR0FBRyxNQUFNLGFBQWEsQ0FBQyxPQUFPLENBQUMsQ0FBQTtJQUM5QyxNQUFNLFlBQVksR0FBRyxlQUFlLENBQUMsT0FBTyxFQUFFLFNBQVMsQ0FBQyxDQUFBO0lBQ3hELE1BQU0sWUFBWSxHQUFHLGVBQWUsQ0FBQyxPQUFPLEVBQUUsWUFBWSxDQUFDLENBQUE7SUFDM0QsTUFBTSxVQUFVLEdBQUcscUJBQXFCLENBQUMsWUFBWSxFQUFFLFlBQVksQ0FBQyxDQUFBO0lBQ3BFLE1BQU0sY0FBYyxHQUFHLHNCQUFzQixDQUFDLFlBQVksRUFBRSxZQUFZLENBQUMsQ0FBQTtJQUV6RSxPQUFPO1FBQ0wsWUFBWTtRQUNaLFlBQVk7UUFDWixVQUFVO1FBQ1YsY0FBYztLQUNmLENBQUE7QUFDSCxDQUFDO0FBeUJELE1BQU0sVUFBVSxjQUFjLENBQzVCLFVBQTJCLEVBQUUsRUFDN0IsVUFBMkIsRUFBRSxFQUM3QixjQUFjLEdBQUcsS0FBSztJQUV0QixNQUFNLG1CQUFtQixHQUFHLGVBQWUsQ0FBQyxPQUFPLEVBQUUsT0FBTyxDQUFDLENBQUE7SUFDN0QsTUFBTSxVQUFVLEdBQUcsY0FBYztRQUMvQixDQUFDLENBQUMsQ0FBQyxHQUFXLEVBQUUsU0FBd0IsRUFBRSxFQUFFLENBQUMsQ0FBQztZQUM1QyxHQUFHO1lBQ0gsU0FBUztTQUNWLENBQUM7UUFDRixDQUFDLENBQUMsQ0FBQyxHQUFXLEVBQUUsRUFBRSxDQUFDLEdBQUcsQ0FBQTtJQUV4QixPQUFPLEtBQUssU0FBUyxDQUFDLENBQUMsS0FBSyxDQUMxQixPQUFpRDtRQUVqRCxNQUFNLEVBQ0osWUFBWSxFQUNaLFlBQVksRUFDWixVQUFVLEVBQ1YsY0FBYyxFQUNmLEdBQUcsTUFBTSxtQkFBbUIsQ0FBQTtRQUM3QixNQUFNLEVBQ0osU0FBUyxFQUNULE9BQU8sRUFDUCxPQUFPLEVBQ1AsSUFBSSxFQUNMLEdBQUcsWUFBWSxDQUFBO1FBQ2hCLElBQUksS0FBYSxDQUFBO1FBQ2pCLElBQUksTUFBd0MsQ0FBQTtRQUM1QyxJQUFJLFNBQXdCLENBQUE7UUFDNUIsSUFBSSxZQUFZLEdBQWdDLEVBQUUsQ0FBQTtRQUNsRCxJQUFJLGNBQWMsR0FBRyxJQUFJLENBQUE7UUFDekIsSUFBSSxNQUFjLENBQUE7UUFDbEIsSUFBSSxjQUFjLEdBQWtCLElBQUksQ0FBQTtRQUN4QyxJQUFJLFlBQVksR0FBRyxJQUFJLENBQUE7UUFFdkIsSUFBSSxLQUFLLEVBQUUsS0FBSyxJQUFJLE9BQU8sRUFBRSxDQUFDO1lBQzVCLE1BQU0sR0FBRyxNQUFNLGVBQWUsQ0FBQyxLQUFLLEVBQUUsU0FBUyxFQUFFLFlBQVksRUFBRSxZQUFZLENBQUMsQ0FBQTtZQUM1RSxTQUFTLEdBQUcsTUFBTSxJQUFJLEtBQUssQ0FBQTtZQUUzQixJQUFJLElBQUksRUFBRSxDQUFDLFNBQVMsQ0FBQyxFQUFFLENBQUM7Z0JBQ3RCLFNBQVE7WUFDVixDQUFDO1lBRUQsMEJBQTBCO1lBQzFCLElBQUksT0FBTyxFQUFFLENBQUM7Z0JBQ1osSUFBSSxNQUFNLEVBQUUsQ0FBQztvQkFDWCxZQUFZLENBQUMsSUFBSSxDQUFDLE1BQU0sQ0FBQyxDQUFBO2dCQUMzQixDQUFDO2dCQUVELElBQUksVUFBVSxDQUFDLFNBQVMsRUFBRSxZQUFZLENBQUMsRUFBRSxDQUFDO29CQUN4QyxjQUFjLEdBQUcsS0FBSyxDQUFBO29CQUN0QixNQUFNLEdBQUcsTUFBTSxjQUFjLENBQUMsWUFBWSxFQUFFLFNBQVMsQ0FBQyxDQUFBO29CQUN0RCxZQUFZLEdBQUcsRUFBRSxDQUFBO29CQUVqQixNQUFNLFVBQVUsQ0FBQyxNQUFNLEVBQUUsU0FBUyxDQUFDLENBQUE7Z0JBQ3JDLENBQUM7WUFDSCxDQUFDO2lCQUFNLENBQUM7Z0JBQ04sSUFBSSxVQUFVLENBQUMsU0FBUyxFQUFFLFlBQVksQ0FBQyxFQUFFLENBQUM7b0JBQ3hDLGNBQWMsR0FBRyxLQUFLLENBQUE7b0JBQ3RCLE1BQU0sR0FBRyxNQUFNLGNBQWMsQ0FBQyxZQUFZLEVBQUUsY0FBYyxDQUFDLENBQUE7b0JBQzNELFlBQVksR0FBRyxFQUFFLENBQUE7b0JBRWpCLElBQUksQ0FBQyxZQUFZLElBQUksT0FBTyxFQUFFLENBQUM7d0JBQzdCLE1BQU0sVUFBVSxDQUFDLE1BQU0sRUFBRSxjQUFjLENBQUMsQ0FBQTtvQkFDMUMsQ0FBQztvQkFFRCxZQUFZLEdBQUcsS0FBSyxDQUFBO29CQUNwQixjQUFjLEdBQUcsU0FBUyxDQUFBO2dCQUM1QixDQUFDO2dCQUVELElBQUksTUFBTSxFQUFFLENBQUM7b0JBQ1gsWUFBWSxDQUFDLElBQUksQ0FBQyxNQUFNLENBQUMsQ0FBQTtnQkFDM0IsQ0FBQztZQUNILENBQUM7UUFDSCxDQUFDO1FBRUQsSUFBSSxDQUFDLE9BQU8sSUFBSSxDQUFDLE9BQU8sSUFBSSxjQUFjLENBQUMsRUFBRSxDQUFDO1lBQzVDLE9BQU07UUFDUixDQUFDO1FBRUQsTUFBTSxHQUFHLE1BQU0sY0FBYyxDQUFDLFlBQVksRUFBRSxjQUFjLENBQUMsQ0FBQTtRQUUzRCxNQUFNLFVBQVUsQ0FBQyxNQUFNLEVBQUUsY0FBYyxDQUFDLENBQUE7SUFDMUMsQ0FBQyxDQUFBO0FBQ0gsQ0FBQztBQUVEOzs7Ozs7R0FNRztBQUNILE1BQU0sVUFBVSxvQkFBb0IsQ0FDbEMsT0FBeUIsRUFDekIsT0FBeUIsRUFDekIsY0FBYyxHQUFHLEtBQUs7SUFFdEIsT0FBTyxTQUFTLENBQUMsSUFBSSxDQUFDLGNBQWMsQ0FBQyxPQUFPLEVBQUUsT0FBTyxFQUFFLGNBQWMsQ0FBQyxDQUFDLENBQUE7QUFDekUsQ0FBQztBQUVEOzs7Ozs7R0FNRztBQUNILE1BQU0sQ0FBQyxLQUFLLFVBQVUsb0JBQW9CLENBQ3hDLE9BQWlELEVBQ2pELE9BQXlCLEVBQ3pCLE9BQXlCO0lBRXpCLE1BQU0sc0JBQXNCLEdBQUcsY0FBYyxDQUFDLE9BQU8sRUFBRSxPQUFPLENBQUMsQ0FBQyxPQUFPLENBQUMsQ0FBQTtJQUN4RSxJQUFJLFNBQVMsR0FBRyxFQUFFLENBQUE7SUFDbEIsSUFBSSxLQUFhLENBQUE7SUFFakIsSUFBSSxLQUFLLEVBQUUsS0FBSyxJQUFJLHNCQUFzQixFQUFFLENBQUM7UUFDM0MsU0FBUyxJQUFJLEtBQUssQ0FBQTtJQUNwQixDQUFDO0lBRUQsT0FBTyxTQUFTLENBQUE7QUFDbEIsQ0FBQyJ9