+++
title = "Role Based Access Control"
weight = 10
sort_by = "weight"
+++

We currently have 3 roles assigned in the system.

## System Administrator

Currently the system administrator is the only person or persons who can manage the models.

There's no need for an administrator to add users or manage teams. Our teams based approach means that assuming someone can access the system then they can start to manage their own team. We feel this scales better than relying on a central entity for user management.

## Team Collaborator

On any team a collaborator can access the console and create prompts, datasets and generally do most things. The only thing they can't do is invite new members to the team.

## Team Administrator

A team administrator role is automatically given to the creator of a team. They have the following permissions.

- Invite new users to the team
- Assign the *Team Administrator* role to a team member.

![Alt text](/teams.png "Start Screen")