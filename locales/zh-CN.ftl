stats-only-dirs = { $total_dirs } 个目录
stats-all = { $total_dirs } 个目录, { $total_files } 个文件

help-usage = 用法：rt [-acdfghilnpqrstuvxACDFJQNSUX] [-L level [-R]] [-H baseHREF] [-T title] [-o filename] [-P pattern] [-I pattern] [--gitignore] [--gitfile[=]file] [--matchdirs] [--metafirst] [--ignore-case] [--nolinks] [--hintro[=]file] [--houtro[=]file] [--inodes] [--device] [--sort[=]<name>] [--dirsfirst] [--filesfirst] [--filelimit #] [--si] [--du] [--prune] [--charset[=]X] [--timefmt[=]format] [--fromfile] [--fromtabfile] [--fflinks] [--info] [--infofile[=]file] [--noreport] [--version] [--help] [--] [directory ...]
help-listing-options = 列表选项
help-all-files = -a 列出所有文件。
help-list-dirs-only = -d 仅列出目录。
help-follow-symlinks = -l 将符号链接视为目录。
help-print-full-path = -f 打印每个文件的完整路径前缀。
help-stay-on-fs = -x 仅停留在当前文件系统。
help-descend-level = -L level 仅递归到指定目录深度。
help-rerun-tree = -R 达到最大目录深度时重新运行树。
help-list-match-pattern = -P pattern 仅列出与给定模式匹配的文件。
help-exclude-match-pattern = -I pattern 不列出与给定模式匹配的文件。
help-filter-gitignore = --gitignore 使用 .gitignore 文件进行过滤。
help-explicit-gitfile = --gitfile X 显式读取 gitignore 文件。
help-ignore-case = --ignore-case 模式匹配时忽略大小写。
help-match-dirs = --matchdirs 在 -P 模式匹配中包含目录名称。
help-meta-first = --metafirst 在每行开头打印元数据。
help-prune-empty-dirs = --prune 从输出中修剪空目录。
help-info-files = --info 打印 .info 文件中找到的文件信息。
help-explicit-infofile = --infofile X 显式读取 info 文件。
help-no-report = --noreport 关闭树列表末尾的文件/目录计数。
help-charset = --charset X 使用 X 字符集进行终端/HTML 和缩进线输出。
help-file-limit = --filelimit # 不递归包含超过 # 文件的目录。
help-output-file = -o filename 输出到文件而不是标准输出。

help-file-options = ------- 文件选项 -------
help-print-nonprintable = -q 将不可打印字符显示为 '?'。
help-print-raw = -N 按原样显示不可打印字符。
help-quote-filenames = -Q 使用双引号引用文件名。
help-print-protections = -p 打印每个文件的权限。
help-display-owner = -u 显示文件所有者或 UID 编号。
help-display-group = -g 显示文件组所有者或 GID 编号。
help-print-size = -s 打印每个文件的字节大小。
help-human-readable-size = -h 以更易读的方式打印大小。
help-si-units = --si 类似 -h，但使用 SI 单位（1000 的幂）。
help-compute-dir-size = --du 按目录内容计算目录大小。
help-print-date = -D 打印最后修改或 (-c) 状态更改的日期。
help-time-format = --timefmt <f> 根据格式 <f> 打印和格式化时间。
help-append-ls = -F 根据 ls -F 附加 '/', '=', '*', '@', '|' 或 '>'。
help-print-inodes = --inodes 打印每个文件的 inode 编号。
help-print-device = --device 打印每个文件所属的设备 ID 编号。

help-sorting-options = ------- 排序选项 -------
help-sort-version = -v 按版本号对文件进行字母数字排序。
help-sort-mtime = -t 按最后修改时间对文件排序。
help-sort-ctime = -c 按最后状态更改时间对文件排序。
help-unsorted = -U 文件保持未排序。
help-reverse-sort = -r 反转排序顺序。
help-dirs-first = --dirsfirst 在文件之前列出目录（-U 禁用）。
help-files-first = --filesfirst 在目录之前列出文件（-U 禁用）。
help-select-sort = --sort X 选择排序：名称、版本、大小、修改时间、状态更改时间。

help-graphics-options = ------- 图形选项 -------
help-no-indent = -i 不打印缩进线。
help-ansi-lines = -A 打印 ANSI 线图形缩进线。
help-console-graphics = -S 使用 CP437（控制台）图形缩进线打印。
help-no-color = -n 始终关闭颜色化（-C 覆盖）。
help-force-color = -C 始终打开颜色化。
help-set-lang = --LANG=X 将输出字符集设置为 LANG 区域设置。

help-xml-html-json = ------- XML/HTML/JSON 选项 -------
help-xml-output = -X 打印树的 XML 表示。
help-json-output = -J 打印树的 JSON 表示。
help-html-output = -H baseHREF 打印 HTML 格式，baseHREF 为顶级目录。
help-html-title = -T string 将默认 HTML 标题和 H1 标头替换为字符串。
help-no-links = --nolinks 关闭 HTML 输出中的超链接。
help-html-intro = --hintro X 使用文件 X 作为 HTML 引言。
help-html-outro = --houtro X 使用文件 X 作为 HTML 结尾。

help-input-options = ------- 输入选项 -------
help-read-from-file = --fromfile 从文件中读取路径（.=stdin）。
help-read-from-tabfile = --fromtabfile 从制表符缩进的文件中读取树（.=stdin）。
help-process-links = --fflinks 使用 --fromfile 时处理链接信息。

help-misc-options = 杂项选项
help-print-version = --version 打印版本并退出。
help-print-help = --help 打印使用方法和此帮助信息并退出。
help-options-terminator = -- 选项处理终止符。