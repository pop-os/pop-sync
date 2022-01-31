# Pop!_Sync Specification

The Pop!_OS sync specification is intended to be simple and quick to implement, with nothing overly complex.

## info.json
info.json is a JSON file containing the data that will be synchronized across devices.

```json
{
	"pop-sync-version": 1,
	"packages": {
		"example": ">=1.0.0",
	},
	"files": {
		"/etc/apt/sources.list.d/organization.list": {
			"add": {
				"source": "package",
				"path": "etc/apt/sources.list.d/organization.list"
			}
		},
		"/etc/blahaj.conf": {
			"add": {
				"source": "url",
				"url": "https://example.com/blahaj.conf",
				"chmod": 0755
			}
		}
	},
	"gconf": {
		"/example/gconf/bool": true,
		"/example/gconf/str": "The Answer",
		"/example/gconf/int": 42,
	}
}
```

`pop-sync-version` (number, required) is the minimum version of the Pop! sync specification that this info.json file is compatible with. The current version is `1`.
`packages` (object, optional) is a list of packages to install, with optionally such as specific versions to install.
`files` (object, optional) is a list of files to copy/delete from the device. Files can be copied from a sync package to the device.
`gconf` (object, optional) is a mapping of gconf keys to values to set on the device.

### packages
TODO

### files
TODO

### gconf
TODO

## sync packages
TODO
