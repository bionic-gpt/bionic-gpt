import { join } from 'path';
import { fileURLToPath } from 'url';
import { readFile } from 'fs/promises';
import Handlebars from 'handlebars';
import { filterRevertedCommitsSync } from 'conventional-commits-filter';
import { getTemplateContext } from './context.js';
const dirname = fileURLToPath(new URL('.', import.meta.url));
/**
 * Load templates from files.
 * @param options
 * @returns Templates strings object.
 */
export async function loadTemplates(options = {}) {
    const [mainTemplate, headerPartial, commitPartial, footerPartial] = await Promise.all([
        options.mainTemplate || readFile(join(dirname, '..', 'templates', 'template.hbs'), 'utf-8'),
        options.headerPartial || readFile(join(dirname, '..', 'templates', 'header.hbs'), 'utf-8'),
        options.commitPartial || readFile(join(dirname, '..', 'templates', 'commit.hbs'), 'utf-8'),
        options.footerPartial || readFile(join(dirname, '..', 'templates', 'footer.hbs'), 'utf-8')
    ]);
    return {
        mainTemplate,
        headerPartial,
        commitPartial,
        footerPartial
    };
}
/**
 * Compile Handlebars templates.
 * @param templates
 * @returns Handlebars template instance.
 */
export function compileTemplates(templates) {
    const { mainTemplate, headerPartial, commitPartial, footerPartial, partials } = templates;
    Handlebars.registerPartial('header', headerPartial);
    Handlebars.registerPartial('commit', commitPartial);
    Handlebars.registerPartial('footer', footerPartial);
    if (partials) {
        Object.entries(partials).forEach(([name, partial]) => {
            if (typeof partial === 'string') {
                Handlebars.registerPartial(name, partial);
            }
        });
    }
    return Handlebars.compile(mainTemplate, {
        noEscape: true
    });
}
/**
 * Create template renderer.
 * @param context
 * @param options
 * @returns Template render function.
 */
export function createTemplateRenderer(context, options) {
    const { ignoreReverted } = options;
    const template = compileTemplates(options);
    return async (commits, keyCommit) => {
        const notes = [];
        const commitsForTemplate = (ignoreReverted
            ? Array.from(filterRevertedCommitsSync(commits))
            : commits).map(commit => ({
            ...commit,
            notes: commit.notes.map((note) => {
                const commitNote = {
                    ...note,
                    commit
                };
                notes.push(commitNote);
                return commitNote;
            })
        }));
        const templateContext = await getTemplateContext(keyCommit, commits, commitsForTemplate, notes, context, options);
        return template(templateContext);
    };
}
//# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoidGVtcGxhdGUuanMiLCJzb3VyY2VSb290IjoiIiwic291cmNlcyI6WyIuLi9zcmMvdGVtcGxhdGUudHMiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IkFBQUEsT0FBTyxFQUFFLElBQUksRUFBRSxNQUFNLE1BQU0sQ0FBQTtBQUMzQixPQUFPLEVBQUUsYUFBYSxFQUFFLE1BQU0sS0FBSyxDQUFBO0FBQ25DLE9BQU8sRUFBRSxRQUFRLEVBQUUsTUFBTSxhQUFhLENBQUE7QUFDdEMsT0FBTyxVQUFVLE1BQU0sWUFBWSxDQUFBO0FBQ25DLE9BQU8sRUFBRSx5QkFBeUIsRUFBRSxNQUFNLDZCQUE2QixDQUFBO0FBVXZFLE9BQU8sRUFBRSxrQkFBa0IsRUFBRSxNQUFNLGNBQWMsQ0FBQTtBQUVqRCxNQUFNLE9BQU8sR0FBRyxhQUFhLENBQUMsSUFBSSxHQUFHLENBQUMsR0FBRyxFQUFFLE1BQU0sQ0FBQyxJQUFJLENBQUMsR0FBRyxDQUFDLENBQUMsQ0FBQTtBQUU1RDs7OztHQUlHO0FBQ0gsTUFBTSxDQUFDLEtBQUssVUFBVSxhQUFhLENBQUMsVUFBNEIsRUFBRTtJQUNoRSxNQUFNLENBQ0osWUFBWSxFQUNaLGFBQWEsRUFDYixhQUFhLEVBQ2IsYUFBYSxDQUNkLEdBQUcsTUFBTSxPQUFPLENBQUMsR0FBRyxDQUFDO1FBQ3BCLE9BQU8sQ0FBQyxZQUFZLElBQUksUUFBUSxDQUFDLElBQUksQ0FBQyxPQUFPLEVBQUUsSUFBSSxFQUFFLFdBQVcsRUFBRSxjQUFjLENBQUMsRUFBRSxPQUFPLENBQUM7UUFDM0YsT0FBTyxDQUFDLGFBQWEsSUFBSSxRQUFRLENBQUMsSUFBSSxDQUFDLE9BQU8sRUFBRSxJQUFJLEVBQUUsV0FBVyxFQUFFLFlBQVksQ0FBQyxFQUFFLE9BQU8sQ0FBQztRQUMxRixPQUFPLENBQUMsYUFBYSxJQUFJLFFBQVEsQ0FBQyxJQUFJLENBQUMsT0FBTyxFQUFFLElBQUksRUFBRSxXQUFXLEVBQUUsWUFBWSxDQUFDLEVBQUUsT0FBTyxDQUFDO1FBQzFGLE9B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