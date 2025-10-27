# semver-diff

> Get the diff type of two [semver](https://github.com/npm/node-semver) versions: `0.0.1 0.0.2` → `patch`

> [!NOTE]
> Deprecated because the `semver` package now supports this built-in.

## Install

```sh
npm install semver-diff
```

## Usage

```js
import semverDiff from 'semver-diff';

semverDiff('1.1.1', '1.1.2');
//=> 'patch'

semverDiff('1.1.1-foo', '1.1.2');
//=> 'prepatch'

semverDiff('0.0.1', '1.0.0');
//=> 'major'

semverDiff('0.0.1-foo', '1.0.0');
//=> 'premajor'

semverDiff('0.0.1', '0.1.0');
//=> 'minor'

semverDiff('0.0.1-foo', '0.1.0');
//=> 'preminor'

semverDiff('0.0.1-foo', '0.0.1-foo.bar');
//=> 'prerelease'

semverDiff('0.1.0', '0.1.0+foo');
//=> 'build'

semverDiff('0.0.1', '0.0.1');
//=> undefined

semverDiff('0.0.2', '0.0.1');
//=> undefined
```

## API

### semverDiff(versionA, versionB)

Returns the difference type between two semver versions, or `undefined` if they are identical or the second one is lower than the first.

Possible values:

- `'major'`,
- `'premajor'`,
- `'minor'`,
- `'preminor'`,
- `'patch'`,
- `'prepatch'`,
- `'prerelease'`,
- `'build'`,
- `undefined`

## Related

- [latest-semver](https://github.com/sindresorhus/latest-semver) - Get the latest stable semver version from an array of versions
- [to-semver](https://github.com/sindresorhus/to-semver) - Get an array of valid, sorted, and cleaned semver versions from an array of strings
- [semver-regex](https://github.com/sindresorhus/semver-regex) - Regular expression for matching semver versions
- [semver-truncate](https://github.com/sindresorhus/semver-truncate) - Truncate a semver version: `1.2.3` → `1.2.0`
