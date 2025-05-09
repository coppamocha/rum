# rum - Remove in a Understanding Manner
Remove files without the pain of loss.

I just mistakenly rm -rfed my project, so presenting rum - a cli utility to delete files without worries of deleting wrong file because you can revert it anytime!

## Usage

``` rum file1 file2 folder1 folder2 ```
all deleted files end up in a seperate trash folder with all of them seperated with a "deletion number" preventing overwrites in case of similar file names.

It creates a .rumrc file in home directory with which you can change the trash directory.
