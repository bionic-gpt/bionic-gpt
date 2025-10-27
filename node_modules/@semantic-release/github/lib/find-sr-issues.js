import { uniqBy } from "lodash-es";
import { ISSUE_ID, RELEASE_FAIL_LABEL } from "./definitions/constants.js";

export default async (octokit, logger, labels, owner, repo) => {
  let issues = [];

  const {
    repository: {
      issues: { nodes: issueNodes },
    },
  } = await octokit.graphql(loadGetSRIssuesQuery, {
    owner,
    repo,
    filter: {
      labels: (labels || []).concat([RELEASE_FAIL_LABEL]),
    },
  });

  issues.push(...issueNodes);

  const uniqueSRIssues = uniqBy(
    issues.filter((issue) => issue.body && issue.body.includes(ISSUE_ID)),
    "number",
  );

  return uniqueSRIssues;
};

/**
 * GraphQL Query to get the semantic-release issues for a repository.
 */
const loadGetSRIssuesQuery = `#graphql
  query getSRIssues($owner: String!, $repo: String!, $filter: IssueFilters) {
    repository(owner: $owner, name: $repo) {
      issues(first: 100, states: OPEN, filterBy: $filter) {
        nodes {
          number
          title
          body
        }
      }
    }
  }
`;
