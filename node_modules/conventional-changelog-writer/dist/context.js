import semver from 'semver';
import { stringify } from './utils.js';
export function getCommitGroups(commits, options) {
    const { groupBy, commitGroupsSort, commitsSort } = options;
    const commitGroups = [];
    const commitGroupsObj = commits.reduce((groups, commit) => {
        const key = commit[groupBy] || '';
        if (groups[key]) {
            groups[key].push(commit);
        }
        else {
            groups[key] = [commit];
        }
        return groups;
    }, {});
    Object.entries(commitGroupsObj).forEach(([title, commits]) => {
        if (commitsSort) {
            commits.sort(commitsSort);
        }
        commitGroups.push({
            title,
            commits
        });
    });
    if (commitGroupsSort) {
        commitGroups.sort(commitGroupsSort);
    }
    return commitGroups;
}
export function getNoteGroups(notes, options) {
    const { noteGroupsSort, notesSort } = options;
    const retGroups = [];
    notes.forEach((note) => {
        const { title } = note;
        let titleExists = false;
        retGroups.forEach((group) => {
            if (group.title === title) {
                titleExists = true;
                group.notes.push(note);
            }
        });
        if (!titleExists) {
            retGroups.push({
                title,
                notes: [note]
            });
        }
    });
    if (noteGroupsSort) {
        retGroups.sort(noteGroupsSort);
    }
    if (notesSort) {
        retGroups.forEach((group) => {
            group.notes.sort(notesSort);
        });
    }
    return retGroups;
}
export function getExtraContext(commits, notes, options) {
    return {
        // group `commits` by `options.groupBy`
        commitGroups: getCommitGroups(commits, options),
        // group `notes` for footer
        noteGroups: getNoteGroups(notes, options)
    };
}
/**
 * Get final context with default values.
 * @param context
 * @param options
 * @returns Final context with default values.
 */
export function getFinalContext(context, options) {
    const finalContext = {
        commit: 'commits',
        issue: 'issues',
        date: options.formatDate(new Date()),
        ...context
    };
    if (typeof finalContext.linkReferences !== 'boolean'
        && (finalContext.repository || finalContext.repoUrl)
        && finalContext.commit
        && finalContext.issue) {
        finalContext.linkReferences = true;
    }
    return finalContext;
}
/**
 * Get context prepared for template.
 * @param keyCommit
 * @param commits
 * @param filteredCommits
 * @param notes
 * @param context
 * @param options
 * @returns Context prepared for template.
 */
export async function getTemplateContext(keyCommit, commits, filteredCommits, notes, context, options) {
    let templateContext = {
        ...context,
        ...keyCommit,
        ...getExtraContext(filteredCommits, notes, options)
    };
    if (keyCommit?.committerDate) {
        templateContext.date = keyCommit.committerDate;
    }
    if (templateContext.version && semver.valid(templateContext.version)) {
        templateContext.isPatch = templateContext.isPatch || semver.patch(templateContext.version) !== 0;
    }
    templateContext = await options.finalizeContext(templateContext, options, filteredCommits, keyCommit, commits);
    options.debug(`Your final context is:\n${stringify(templateContext)}`);
    return templateContext;
}
//# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiY29udGV4dC5qcyIsInNvdXJjZVJvb3QiOiIiLCJzb3VyY2VzIjpbIi4uL3NyYy9jb250ZXh0LnRzIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiJBQUFBLE9BQU8sTUFBTSxNQUFNLFFBQVEsQ0FBQTtBQVUzQixPQUFPLEVBQUUsU0FBUyxFQUFFLE1BQU0sWUFBWSxDQUFBO0FBRXRDLE1BQU0sVUFBVSxlQUFlLENBQzdCLE9BQWlCLEVBQ2pCLE9BQW1GO0lBRW5GLE1BQU0sRUFDSixPQUFPLEVBQ1AsZ0JBQWdCLEVBQ2hCLFdBQVcsRUFDWixHQUFHLE9BQU8sQ0FBQTtJQUNYLE1BQU0sWUFBWSxHQUEwQixFQUFFLENBQUE7SUFDOUMsTUFBTSxlQUFlLEdBQUcsT0FBTyxDQUFDLE1BQU0sQ0FBMkIsQ0FBQyxNQUFNLEVBQUUsTUFBTSxFQUFFLEVBQUU7UUFDbEYsTUFBTSxHQUFHLEdBQUcsTUFBTSxDQUFDLE9BQU8sQ0FBVyxJQUFJLEVBQUUsQ0FBQTtRQUUzQyxJQUFJLE1BQU0sQ0FBQyxHQUFHLENBQUMsRUFBRSxDQUFDO1lBQ2hCLE1BQU0sQ0FBQyxHQUFHLENBQUMsQ0FBQyxJQUFJLENBQUMsTUFBTSxDQUFDLENBQUE7UUFDMUIsQ0FBQzthQUFNLENBQUM7WUFDTixNQUFNLENBQUMsR0FBRyxDQUFDLEdBQUcsQ0FBQyxNQUFNLENBQUMsQ0FBQTtRQUN4QixDQUFDO1FBRUQsT0FBTyxNQUFNLENBQUE7SUFDZixDQUFDLEVBQUUsRUFBRSxDQUFDLENBQUE7SUFFTixNQUFNLENBQUMsT0FBTyxDQUFDLGVBQWUsQ0FBQyxDQUFDLE9BQU8sQ0FBQyxDQUFDLENBQUMsS0FBSyxFQUFFLE9BQU8sQ0FBQyxFQUFFLEVBQUU7UUFDM0QsSUFBSSxXQUFXLEVBQUUsQ0FBQztZQUNoQixPQUFPLENBQUMsSUFBSSxDQUFDLFdBQVcsQ0FBQyxDQUFBO1FBQzNCLENBQUM7UUFFRCxZQUFZLENBQUMsSUFBSSxDQUFDO1lBQ2hCLEtBQUs7WUFDTCxPQUFPO1NBQ1IsQ0FBQyxDQUFB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