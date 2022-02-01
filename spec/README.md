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
`packages` is an object of packages to install, along with version requirements for the package. The package name is the key, and the value is the version requirement. The version requirement can be any valid Debian version range.

Valid examples:
```json
{
	"libvtk6-dev": ">= 6.0.0",
	"libvtk6-dev": "<< 6.2.0",
	"libpython2.7-dev": "*"
}
```

### files
`files` is an object containing files to copy, download, or delete. The key is the path to the file, and the value is an object containing the action to perform.

Valid actions are `add` and `remove`.

#### add

Adding a file can be done by copying it from the sync package, or downloading it from a URL.

If you wish to copy it from the package, `source` should be `"package"`, and `path` should be set to the relative path of the file in the package.

If you wish to download it from the internet, then `source` should be `"url"`, and `url` should be set to the URL to download from.

Optionally, the permission bits can be set with `chmod`, and the owners set with `uid` and `gid` or `user` and `group`.

`add` examples:
```json
{
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
			"chmod": 0755,
			"user": "root"
		}
	}
}
```

#### remove

Removing a file is much simpler. Just set `"remove": true` in the file's object.

`remove` example:
```json
{
	"/etc/badconfig.conf", {
		"remove": true
	}
}
```


### gconf
TODO

## sync packages
TODO
