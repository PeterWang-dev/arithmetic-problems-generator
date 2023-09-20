# Contributing Guidelines

The contribution of this project must follow the guidelines below.

In the beginning, we announce that **English is the only language** that is allowed
in the whole workflow, as well as the code, comments and inline documentation.
Because complex characters in other languages bring in encoding problems.

## Coding Style

We use [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
as our coding style, which is recommended by Rust RFCs.

Contributors should **strictly** follow the guidelines when writing code.

## Git Workflow

We use git as our version control system, and follow the git workflow below.

1. Before fixing a bug or implementing a new feature,
contributers should **create an issue** in the issue tracker.
The template of the issue has been involved in [README.md](README.md#issue-template).

2. Contributers should **create a branch** for the issue.
Naming convention of the branch is stated [below](#branch-conventions).

3. When making progress, please commit your changes instantly.
Specifically, just commit **change of one function in one commit** .
The format of the commit message is stated [below](#commit-message).

4. When the issue is fixed or the feature is implemented,
please **create a pull request** for the branch.
Refer to the [following section](#pull-request) for details.

5. Please do a **self-review** first after creating a pull request.
The assignee of the pull request will review the code and merge it if it is OK.
Discussing if necessary.

### Branch Conventions

Three main kinds of branches are used in this project:

- `main` : The main branch of the project.

  `main` branch should be **stable** and **ready to release** .
  A pull request tagged `[Release]`
  should be created to merge the `dev` branch into `main` branch
  when a new version is ready to release.
  Release tags should be created on the `main` branch.
  Hotfixes should be merged into `main` branch directly.

- `dev` : The **base branch** of the **development** .

  `dev` branch is the base branch of the development.
  The features and bug fixes should be merged into `dev` branch via pull requests.
  When `dev` branch is in a stable state,
  a pull request tagged `[Release]` will be created to merge it into `main` branch.

- Other development branches: The branches for developing created from `dev` branch.

The naming convention of the development branch is:

`<Type>/<Issue Number>-<Short Description>`

#### Type

There are mainly 6 types of development branches:

- `hotfix` : **Hotfixes** for the `main` branch.

  When encountering a bug in the `main` branch,
  which should be fixed immediately, a `hotfix` branch should be created.
  After the fix, the branch should be merged into `main` branch directly.

- `feature` : New **features** or **enhancements** .

  When implementing a new feature or enhancement for existing features,
  a `feature` branch should be created.
  After the feature is implemented, the branch should be merged into `dev` branch.

- `bugfix` : Bug **fixes** .

  When fixing a bug, a `bugfix` branch should be created.
  After the bug is fixed, the branch should be merged into `dev` branch.

- `documentation` : **Documentation** for project.

  When writing or improving documentation, a `doc` branch should be created.
  When using `documentation` branch, developers should not change the code.
  After the documentation is finished, the branch should be merged into `dev` branch.

- `chore` : **Other** chores except for the above.

  When doing other chores, a `chore` branch should be created.
  Chore is defined as something that is not a feature, bugfix, or documentation,
  just like workspace configuration, CI/CD configuration, etc.
  After the chore is finished, the branch should be merged into `dev` branch.

#### Issue Number

The `<Issue Number>` is the number of the issue in the issue tracker.
**Before** developing a feature or fixing a bug, an **issue should be created** .

#### Short Description

The `<Short Description>` is a short description of the issue,
which should be a short phrase.

**Dash** ( `-` ) should be used to separate issue number and short description,
and **underscore** ( `_` ) should be used to separate words in the short description.

### Commit Message

We refered to [this article](https://www.freecodecamp.org/news/how-to-write-better-git-commit-messages/)
and [Google Blocky Git Commit Guide](https://developers.google.com/blockly/guides/contribute/get-started/commits)
when specifying our commit message guidelines.

The format of the commit message is:

```text
<type>(<scope>): <subject>

<body>

<footer>
```

#### Type

The `<type>` is the type of the commit, which **should be one of the following** :

- `feat` : a new feature is introduced with the changes
- `fix` : a bug fix has occurred
- `chore` : changes that do not relate to a fix or feature
  and don't modify src or test files (for example updating dependencies)
- `refactor` : refactored code that neither fixes a bug nor adds a feature
- `docs` : updates to documentation such as a the README or other markdown files
- `style` : changes that do not affect the meaning of the code,
  likely related to code formatting such as white-space, missing semi-colons, and so on.
- `test` : including new or correcting previous tests
- `perf` : performance improvements
- `ci` : continuous integration related
- `build` : changes that affect the build system or external dependencies
- `revert` : reverts a previous commit

**Breaking changes**

Commits that make breaking changes should append a `!` after the type of the commit.
Breaking changes are **changes that may break** the other part (including user),
causing developers or users to have to do extra work.

#### Scope

The `<scope>` is *optional* , which is the affected area of the commit.

There is no strict rule for the scope,
but it should be a short phrase describing the affected area precisely,
and should be **enclosed in parentheses** ( `()` ).

Use the name of the affected function
(not the function signature, but an abstract name of the part of program)
is usually a good choice.

#### Subject

The `<subject>` is a short description of the commit,
which should be a sentence with following **rules** :

- **DO NOT** use capital letter at the beginning.
- **DO NOT** end with a period ( `.` ).
- **Use** imperative, present tense: "change" not "changed" nor "changes".
- **No more than 50 characters**.

#### Body

The `<body>` is *optional* , which is a long description of the commit.

When changes are too complex to describe in the subject,
please use markdown list to describe the changes in the body in an organized way.

#### Footer

The `<footer>` is *optional* , which is used to reference issues.

According to the [GitHub Docs](https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/using-keywords-in-issues-and-pull-requests),
There is a few keywords that can be used to reference issues, even close them.

We use a subset of the keywords:

- `Closes` : the commit **closes** the issue.
  **DO NOT** use this keyword because we want GitHub to close the issue automatically.
- `Fixes` : the commit **fixes** the issue.
  When commit finally fix an issue (any kind including bug, feature request, etc),
  use this keyword to reference the issue instaed of `Closes` .
- `Resolves` : the commit **resolves** the issue.
  Use this key word when commit resolve an issue,
  especially when the commit is not the only commit that resolve the issue.

### Pull Request

When the development branch is completed,
a pull request should be created to merge the branch into `dev` branch.

When `dev` branch is in a stable state,
a pull request tagged `[Release]` will be created to merge it into `main` branch.

`main` branch is **strictly protected** .
**No one** can push to `main` branch directly except for the initializations of the repository.

`dev` branch is **protected** .
Only repository **owner or ones with permission** can push to `dev` branch directly.

#### Title

The title of the pull request should be:

`[<Type>] <Short Description> #<Issue number>`

The `<Type>` is the type of the development branch.

And the `<Short Description>` is the short description of the issue.

`<Issue number>` is the number of the issue in the issue tracker,
referanced by the pull request via `#` sign.

An exception is that when merging `dev` branch into `main` branch.
The title should be `[Release] <Package Name> <Version>` at that time.

#### Description

The description of the pull request should involve the **hotspots** of the changes.
If you want **reviewers** to focus on some parts of the changes,
please use markdown **checklist** to state them out.

When encountering a release merge,
the description should be the **release notes** of the version.
A simple release note template is:

```markdown
# CHANGELOG

## Bug Fixes

- #<Issue Number>

## Enhancements

- #<Issue Number>

## Features

- #<Issue Number>

## <Some Other Categories>

...
```

#### Source and Target Branch

|           Source           | Target |
|:--------------------------:|:------:|
|             dev            |  main  |
|           hotfix           |  main  |
| other development branches |   dev  |

#### Conflicts

Conflicts should be resolved before merging the pull request.

There is a recommended way to resolve conflicts:

1. Merge the **target branch** into the **source branch** .
2. Resolve conflicts via tools and commit.
3. Merge the **source branch** into the **target branch** if everything is OK.

Use `chore` in merge commit message and make the first letter of the subject decapitalized.

#### Merge

Reviewers should **review all the changes** in the pull request before merging it.

When merging a pull request, always use **squash and merge** in GitHub.

The title of the merge commit should be **the title of the pull request** .

The message of the merge commit should be **the changes of the pull request** ,
which can be generated by GitHub automatically.

When merging a pull request tagged `[Release]` ,
the title of the merge commit should be `[Release] <Package Name> <Version>` ,
and the message of the merge commit should be **the release notes** of the version.
