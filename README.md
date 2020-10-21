# RustRsync Project

Automation backups with rsync using a toml file with selected folders

## Warning << This is a personal project >> Warning

> It is still in development and need a lot of improves

### TODO:

- Validate that the file contains a valid configuration
  and manage the Error panics.
- Add a flag --init for create a basic structure toml file.
- code refactoring

__The program works but check the code for understanding its operation
and avoid errors in the backup.__

The file toml structure is:

```
[origin_folder]
origin = "/home/user/"

[destination_folder]
destination = "/run/media/user/externalStorage/"

[folders]
folders = [
  "Documents/",
  "Music/",
  "Videos/",
  "Picture/"
]
```

1. The first key is the origin_folder.
2. The second key is the destination_folder.
3. The third key is the folders to be backed up.

for example, this would be the first command generated:
```
rsync -rtvu --delete /home/user/Documents/ /run/media/user/externalStorage/Documents/
```

__Don't forget to put the / at the end of the folder lines and start the folder 
names to be backed up without / to avoid rsync duplication problems.__

## Use

```
$ rustrsync -f folder_backup.toml
$ rustrsync --file folder_backup.toml
$ rustrsync --help
$ rustrsync --version
```

## License

This source code is released under MIT License.
