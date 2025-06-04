We currently have 3 roles assigned in the system.

## System Administrator

Currently the system administrator is the only person or persons who can manage the models.

There's no need for an administrator to add users or manage teams. Our teams based approach means that assuming someone can access the system then they can start to manage their own team. We feel this scales better than relying on a central entity for user management.

## Team Collaborator

On any team a collaborator can access the console and create prompts, datasets and generally do most things. The only thing they can't do is invite new members to the team.

## Team Manager

A team manager role is automatically given to the creator of a team. They have the following permissions.

- Invite new users to the team
- Assign the *Team Administrator* role to a team member.

![Alt text](/landing-page/teams.png "Start Screen")

## Restricting the Team Collaborator

All the available permissions are stored in the [Enum](https://www.postgresql.org/docs/current/datatype-enum.html) called `permission`.

### To view the permissions.

```sql
SELECT enum_range(NULL::permission);

{InvitePeopleToTeam,ViewCurrentTeam,ViewPrompts,ManagePipelines,
ViewDatasets,ManageDatasets,CreateApiKeys,ViewAuditTrail,SetupMod
els}
```

### View all the roles

```sql
bionicgpt=# SELECT enum_range(NULL::role);
                   enum_range                   
------------------------------------------------
 {TeamManager,Collaborator,SystemAdministrator}
(1 row)

bionicgpt=# 
```

### View how permissions are assigned to roles

```sql
bionicgpt=# select * from roles_permissions;
        role         |     permission     
---------------------+--------------------
 TeamManager         | InvitePeopleToTeam
 SystemAdministrator | ViewAuditTrail
 SystemAdministrator | SetupModels
 Collaborator        | ViewCurrentTeam
 Collaborator        | ViewPrompts
 Collaborator        | ManageDatasets
 Collaborator        | ViewDatasets
 Collaborator        | CreateApiKeys
(8 rows)
```

So finally, any permissions you don't want **Team Collaborators** to have, you could transfer to the **System Administrator**.

### Example - Only a System Administrator can manage teams and API Keys

```sql
UPDATE roles_permissions SET role = 'SystemAdministrator' where permission = 'CreateApiKeys';
UPDATE roles_permissions SET role = 'SystemAdministrator' where permission = 'ViewCurrentTeam';
UPDATE roles_permissions SET role = 'SystemAdministrator' where permission = 'InvitePeopleToTeam';
```

This would leave the user interface looking like the one below for any user that is not **System Administrator**.

![Alt text](rbac.png "Start Screen")