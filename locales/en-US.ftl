# count:folder & file
stats-only-dirs = { $total_dirs } director(ies)
stats-all = { $total_dirs } director(ies), { $total_files } file(s)

# help
help-usage = usage: rt [-acdfghilnpqrstuvxACDFJQNSUX] [-L level [-R]] [-H baseHREF]
    | [-T title] [-o filename] [-P pattern] [-I pattern] [--gitignore]
    | [--gitfile[=]file] [--matchdirs] [--metafirst] [--ignore-case]
    | [--nolinks] [--hintro[=]file] [--houtro[=]file] [--inodes] [--device]
    | [--sort[=]<name>] [--dirsfirst] [--filesfirst] [--filelimit #] [--si]
    | [--du] [--prune] [--charset[=]X] [--timefmt[=]format] [--fromfile]
    | [--fromtabfile] [--fflinks] [--info] [--infofile[=]file] [--noreport]
    | [--version] [--help] [--] [directory ...]

help-listing-options = Listing options
help-all-files = -a            All files are listed.
help-list-dirs-only = -d            List directories only.
help-follow-symlinks = -l            Follow symbolic links like directories.
help-print-full-path = -f            Print the full path prefix for each file.
help-stay-on-fs = -x            Stay on current filesystem only.
help-descend-level = -L=<level>      Descend only level directories deep.
help-rerun-tree = -R            Rerun tree when max dir level reached.
help-list-match-pattern = -P pattern    List only those files that match the pattern given.
help-exclude-match-pattern = -I pattern    Do not list files that match the given pattern.
help-filter-gitignore = --gitignore   Filter by using .gitignore files.
help-explicit-gitfile = --gitfile X   Explicitly read gitignore file.
help-ignore-case = --ignore-case Ignore case when pattern matching.
help-match-dirs = --matchdirs   Include directory names in -P pattern matching.
help-meta-first = --metafirst   Print meta-data at the beginning of each line.
help-prune-empty-dirs = --prune       Prune empty directories from the output.
help-info-files = --info        Print information about files found in .info files.
help-explicit-infofile = --infofile X Explicitly read info file.
help-no-report = --noreport    Turn off file/directory count at end of tree listing.
help-charset = --charset X    Use charset X for terminal/HTML and indentation line output.
help-file-limit = --filelimit # Do not descend dirs with more than # files in them.
help-output-file = -o filename   Output to file instead of stdout.

help-file-options = File options
help-print-nonprintable = -q            Print non-printable characters as '?'.
help-print-raw = -N            Print non-printable characters as is.
help-quote-filenames = -Q            Quote filenames with double quotes.
help-print-protections = -p            Print the protections for each file.
help-display-owner = -u            Displays file owner or UID number.
help-display-group = -g            Displays file group owner or GID number.
help-print-size = -s            Print the size in bytes of each file.
help-human-readable-size = -h            Print the size in a more human readable way.
help-si-units = --si          Like -h, but use in SI units (powers of 1000).
help-compute-dir-size = --du          Compute size of directories by their contents.
help-print-date = -D            Print the date of last modification or (-c) status change.
help-time-format = --timefmt <f> Print and format time according to the format <f>.
help-append-ls = -F            Appends '/', '=', '*', '@', '|' or '>' as per ls -F.
help-print-inodes = --inodes      Print inode number of each file.
help-print-device = --device      Print device ID number to which each file belongs.

help-sorting-options = Sorting options
help-sort-version = -v            Sort files alphanumerically by version.
help-sort-mtime = -t            Sort files by last modification time.
help-sort-ctime = -c            Sort files by last status change time.
help-unsorted = -U            Leave files unsorted.
help-reverse-sort = -r            Reverse the order of the sort.
help-dirs-first = --dirsfirst   List directories before files (-U disables).
help-files-first = --filesfirst  List files before directories (-U disables).
help-select-sort = --sort X      Select sort: name,version,size,mtime,ctime.

help-graphics-options = Graphics options
help-no-indent = -i            Don't print indentation lines.
help-ansi-lines = -A            Print ANSI lines graphic indentation lines.
help-console-graphics = -S            Print with CP437 (console) graphics indentation lines.
help-no-color = -n            Turn colorization off always (-C overrides).
help-force-color = -C            Turn colorization on always.
help-set-lang = --LANG=X      Set output character set to LANG locale.

help-xml-html-json = XML/HTML/JSON options
help-xml-output = -X            Prints out an XML representation of the tree.
help-json-output = -J            Prints out an JSON representation of the tree.
help-html-output = -H baseHREF   Prints out HTML format with baseHREF as top directory.
help-html-title = -T string     Replace the default HTML title and H1 header with string.
help-no-links = --nolinks     Turn off hyperlinks in HTML output.
help-html-intro = --hintro X    Use file X as the HTML intro.
help-html-outro = --houtro X    Use file X as the HTML outro.

help-input-options = Input options
help-read-from-file = --fromfile    Reads paths from files (.=stdin)
help-read-from-tabfile = --fromtabfile Reads trees from tab indented files (.=stdin)
help-process-links = --fflinks     Process link information when using --fromfile.

help-misc-options = Miscellaneous options
help-print-version = --version     Print version and exit.
help-print-help = --help        Print usage and this help message and exit.
help-options-terminator = --            Options processing terminator.

# level
level-missing = Missing argument to -L option
level-must-1 = Value for -L option must be >= 1
level-bad = Bad argument to -L option