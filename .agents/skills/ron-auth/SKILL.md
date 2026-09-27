---
name: ron-auth
description: Implement identity and authorization boundaries in Rust on Nails applications. Use for verified identity, team membership, permissions, ownership, and authenticated transaction context.
---

# Rust on Nails Authentication and Authorization

Inspect the destination project's identity provider, middleware, request extractor,
permissions, and database policies. Do not assume header names or helper functions.

## Identity boundary

Use an identity already verified by trusted authentication middleware. Decoding a
JWT payload does not verify its signature, issuer, audience, or expiry. Forwarded
identity headers are trustworthy only when an authenticated upstream controls them
and clients cannot bypass it or inject identity headers. Never add a development
identity override as a production authentication mechanism.

## Authorization

- Resolve user identity from authentication, not form or tool input.
- Authorize the selected team before loading protected data. Team IDs from routes
  remain untrusted selectors until checked against membership.
- Check operation permissions and record scope. Being able to read a shared record
  does not imply permission to edit it; membership in two teams does not permit
  substituting either team's record under the other's route.
- Enforce ownership/permissions in the handler and persistence layer. UI checks only
  hide controls. Keep record authorization and writes atomic.
- For PostgreSQL RLS, set context transaction-locally before protected queries on the
  same connection. Never leave per-user state on a pooled connection after a request.
  Inspect table-owner/BYPASSRLS behavior and the application's actual database role.
- Treat background privileged access as a separate workflow, not a user-facing bypass.

## Local contract example

[authorization.rs](examples/authorization.rs) defines verified identity, item scope,
and explicit team/owner checks using only the Rust standard library. It is a policy
example: all team members may read; only owners may mutate. Populate identity from
verified middleware and obtain item scope from trusted storage. Apply equivalent
predicates in the same database statement/transaction as mutations.

No framework extractor or identity-provider setup is assumed. Adapt the policy to
real roles without treating the example's owner-only rule as a universal requirement.

## Verification

Test absent/invalid identity at middleware, nonmember teams, wrong owner, swapped team
IDs even for a multi-team member, and allowed read/write paths. Test RLS context isolation
across pooled requests. Optional companion: `ron-database`, if installed.
