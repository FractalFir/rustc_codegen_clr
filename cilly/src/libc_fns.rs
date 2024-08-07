/// A list of all functions which are redirected to system libc.
pub const LIBC_FNS: &[&str] = &[
    "__errno_location",
    "__xpg_strerror_r",
    "a64l",
    "abort",
    "abs",
    "accept4",
    "accept",
    "acct",
    "alarm",
    "alphasort",
    "asctime",
    "atof",
    "atoi",
    "atol",
    "atoll",
    "bcopy",
    "bind",
    "bindresvport",
    "bsearch",
    "c8rtomb",
    "c16rtomb",
    "c32rtomb",
    "capget",
    "capset",
    "catclose",
    "catgets",
    "catopen",
    "cfgetispeed",
    "cfgetospeed",
    "cfmakeraw",
    "cfsetispeed",
    "cfsetospeed",
    "cfsetspeed",
    "chdir",
    "chflags",
    "chroot",
    "clearerr",
    "clock",
    "clock_gettime",
    "close",
    "closelog",
    "confstr",
    "connect",
    "ctermid",
    "ctime",
    "cuserid",
    "daemon",
    "difftime",
    "dirfd",
    "dirname",
    "div",
    "dlsym",
    "dprintf",
    "drand48",
    "dysize",
    "ecvt",
    "endaliasent",
    "endfsent",
    "endgrent",
    "endhostent",
    "endnetent",
    "endnetgrent",
    "endprotoent",
    "endpwent",
    "endrpcent",
    "endservent",
    "endsgent",
    "endspent",
    "endusershell",
    "endutxent",
    "erand48",
    "err",
    "errx",
    "eventfd",
    "execl",
    "execle",
    "execlp",
    "execv",
    "execveat",
    "execvp",
    "exit",
    "fallocate64",
    "fallocate",
    "fchflags",
    "fchmodat",
    "fchownat",
    "fcntl",
    "fcvt",
    "fdatasync",
    "fdopendir",
    "fexecve",
    "ffs",
    "ffsll",
    "fgetgrent",
    "fgetpwent",
    "fgetsgent",
    "fgetspent",
    "fgetws",
    "fgetxattr",
    "flistxattr",
    "fmtmsg",
    "fprintf",
    "fputc",
    "fputwc",
    "fputws",
    "free",
    "freeaddrinfo",
    "fremovexattr",
    "freopen64",
    "freopen",
    "fscanf",
    "fsconfig",
    "fseek",
    "fsetxattr",
    "fsmount",
    "fsopen",
    "fspick",
    "fstat64",
    "fsync",
    "ftime",
    "ftok",
    "ftw64",
    "ftw",
    "fwide",
    "fwscanf",
    "gai_strerror",
    "gcvt",
    "getaddrinfo",
    "getaliasbyname",
    "getaliasent",
    "getchar",
    "getcwd",
    "getdate",
    "getdirentries64",
    "getdirentries",
    "getdomainname",
    "getentropy",
    "getenv",
    "getfsent",
    "getfsfile",
    "getfsspec",
    "getgrent",
    "getgrgid",
    "getgrnam",
    "getgrouplist",
    "gethostbyaddr",
    "gethostbyname2",
    "gethostbyname",
    "gethostent",
    "gethostid",
    "getipv4sourcefilter",
    "getloadavg",
    "getlogin",
    "getmntent",
    "getnameinfo",
    "getnetbyaddr",
    "getnetbyname",
    "getnetent",
    "getnetgrent",
    "getopt",
    "getpass",
    "getpeername",
    "getpgrp",
    "getpid",
    "getprotobyname",
    "getprotobynumber",
    "getprotoent",
    "getpwent",
    "getpwnam",
    "getpwuid",
    "getrandom",
    "getrpcbyname",
    "getrpcbynumber",
    "getrpcent",
    "getservbyname",
    "getservbyport",
    "getservent",
    "getsgent",
    "getsgnam",
    "getsid",
    "getsockname",
    "getsourcefilter",
    "getspent",
    "getspnam",
    "getsubopt",
    "getusershell",
    "getutmp",
    "getutmpx",
    "getutxent",
    "getutxid",
    "getutxline",
    "getw",
    "getwchar",
    "getwd",
    "getxattr",
    "globfree64",
    "globfree",
    "gmtime",
    "gnu_get_libc_version",
    "grantpt",
    "gtty",
    "hcreate",
    "herror",
    "hsearch",
    "hstrerror",
    "htonl",
    "htons",
    "iconv",
    "initgroups",
    "innetgr",
    "insque",
    "ioctl",
    "ioperm",
    "iopl",
    "iruserok",
    "isalnum",
    "isalpha",
    "isascii",
    "isatty",
    "isblank",
    "iscntrl",
    "isdigit",
    "isfdtype",
    "isgraph",
    "islower",
    "isprint",
    "ispunct",
    "isspace",
    "isupper",
    "isxdigit",
    "jrand48",
    "killpg",
    "klogctl",
    "l64a",
    "labs",
    "lchmod",
    "lcong48",
    "ldiv",
    "lfind",
    "lgetxattr",
    "linkat",
    "listen",
    "listxattr",
    "llabs",
    "lldiv",
    "llistxattr",
    "localtime",
    "lockf",
    "lrand48",
    "lremovexattr",
    "lsearch",
    "lseek64",
    "lsetxattr",
    "malloc",
    "mblen",
    "mbrtoc8",
    "mbrtoc16",
    "mbrtoc32",
    "mbstowcs",
    "mbtowc",
    "mcheck",
    "mcheck_check_all",
    "memcmp",
    "memcpy",
    "memfrob",
    "memmove",
    "memset",
    "mincore",
    "mkdir",
    "mkdirat",
    "mkdtemp",
    "mkfifo",
    "mkfifoat",
    "mkostemp64",
    "mkostemp",
    "mkostemps64",
    "mkostemps",
    "mkstemp64",
    "mkstemp",
    "mkstemps64",
    "mkstemps",
    "mktime",
    "mlock2",
    "mlock",
    "mlockall",
    "mprobe",
    "mrand48",
    "msgget",
    "msync",
    "mtrace",
    "munlock",
    "munlockall",
    "muntrace",
    "nanosleep",
    "nice",
    "nrand48",
    "open64",
    "open",
    "openat64",
    "openlog",
    "perror",
    "pipe2",
    "ppoll",
    "preadv2",
    "preadv64",
    "preadv64v2",
    "preadv",
    "printf",
    "prlimit64",
    "prlimit",
    "psiginfo",
    "psignal",
    "pthread_atfork",
    "ptrace",
    "ptsname",
    "putchar",
    "putenv",
    "putgrent",
    "putpwent",
    "puts",
    "putsgent",
    "putspent",
    "pututxline",
    "putw",
    "putwc",
    "putwchar",
    "pwritev2",
    "pwritev64",
    "pwritev64v2",
    "pwritev",
    "qecvt",
    "qfcvt",
    "qgcvt",
    "qsort",
    "quotactl",
    "raise",
    "rand",
    "rcmd",
    "read",
    "readlink",
    "readlinkat",
    "readv",
    "realloc",
    "reboot",
    "recv",
    "recvfrom",
    "remove",
    "removexattr",
    "remque",
    "rename",
    "rewind",
    "rexec",
    "rpmatch",
    "rresvport",
    "ruserok",
    "ruserpass",
    "scandir",
    "scandirat64",
    "scanf",
    "sched_getaffinity",
    "sched_yield",
    "seed48",
    "seekdir",
    "semget",
    "semop",
    "send",
    "sendfile64",
    "sendfile",
    "sendto",
    "setaliasent",
    "setbuf",
    "setdomainname",
    "setegid",
    "setenv",
    "seteuid",
    "setfsent",
    "setfsgid",
    "setfsuid",
    "setgrent",
    "setgroups",
    "sethostent",
    "sethostid",
    "sethostname",
    "setipv4sourcefilter",
    "setjmp",
    "setlinebuf",
    "setlocale",
    "setlogin",
    "setlogmask",
    "setnetent",
    "setnetgrent",
    "setns",
    "setpgrp",
    "setprotoent",
    "setpwent",
    "setrpcent",
    "setservent",
    "setsgent",
    "setsockopt",
    "setsourcefilter",
    "setspent",
    "setusershell",
    "setutxent",
    "setxattr",
    "sgetsgent",
    "sgetspent",
    "shmat",
    "shmdt",
    "shmget",
    "shutdown",
    "sigaddset",
    "sigandset",
    "sigdelset",
    "sigemptyset",
    "sigfillset",
    "siggetmask",
    "sighold",
    "sigignore",
    "siginterrupt",
    "sigisemptyset",
    "sigismember",
    "signalfd",
    "sigorset",
    "sigpending",
    "sigrelse",
    "sigset",
    "sigstack",
    "sockatmark",
    "socket",
    "socketpair",
    "splice",
    "sprintf",
    "srand48",
    "sscanf",
    "stat64",
    "statx",
    "strcat",
    "strchr",
    "strcmp",
    "strcoll",
    "strcpy",
    "strcspn",
    "strerror",
    "strfmon",
    "strfromd",
    "strfromf128",
    "strfromf",
    "strfroml",
    "strfry",
    "strftime",
    "strlen",
    "strncat",
    "strncmp",
    "strncpy",
    "strnlen",
    "strpbrk",
    "strptime",
    "strrchr",
    "strsignal",
    "strspn",
    "strstr",
    "strtod",
    "strtof128",
    "strtof",
    "strtok",
    "strtold",
    "strxfrm",
    "stty",
    "swab",
    "swprintf",
    "swscanf",
    "symlinkat",
    "sync",
    "syncfs",
    "syscall",
    "syslog",
    "tcflow",
    "tcflush",
    "tcgetpgrp",
    "tcgetsid",
    "tcsendbreak",
    "tcsetattr",
    "tee",
    "telldir",
    "tempnam",
    "timegm",
    "tmpfile64",
    "tmpnam",
    "toascii",
    "tolower",
    "toupper",
    "ttyname",
    "ttyslot",
    "ualarm",
    "ungetc",
    "ungetwc",
    "unlinkat",
    "unlockpt",
    "unshare",
    "updwtmpx",
    "usleep",
    "utime",
    "utmpxname",
    "verr",
    "verrx",
    "versionsort",
    "vfprintf",
    "vhangup",
    "vlimit",
    "vmsplice",
    "vprintf",
    "vwarn",
    "vwarnx",
    "vwprintf",
    "vwscanf",
    "warn",
    "warnx",
    "wcscspn",
    "wcsdup",
    "wcsftime",
    "wcsncat",
    "wcsncmp",
    "wcspbrk",
    "wcsrchr",
    "wcsspn",
    "wcsstr",
    "wcstod",
    "wcstof128",
    "wcstof",
    "wcstok",
    "wcstold",
    "wcstombs",
    "wcswidth",
    "wcsxfrm",
    "wctob",
    "wctomb",
    "wcwidth",
    "wordexp",
    "wordfree",
    "wprintf",
    "write",
    "writev",
    "wscanf", /* "pthread_attr_destroy",
              "pthread_attr_init",
              "pthread_attr_setstacksize",
              "pthread_create",
              "pthread_detach",
              "pthread_join",
              "pthread_self",
              "pthread_setname_np",*/
];
pub const LIBC_MODIFIES_ERRNO: &[&str] = &[
    "bind",
    "chdir",
    "clock_gettime",
    "create",
    "fdopendir",
    "fstat64",
    "getcwd",
    "getpeername",
    "getpid",
    "getrandom",
    "getsockname",
    "ioctl",
    "isatty",
    "lseek64",
    "mkdir",
    "open64",
    "open",
    "openat64",
    "pipe2",
    "pthread_atfork",
    "pthread_attr_destroy",
    "pthread_attr_init",
    "pthread_attr_setstacksize",
    "pthread_create",
    "pthread_detach",
    "pthread_join",
    "pthread_self",
    "pthread_setname_np",
    "read",
    "readlink",
    "readlinkat",
    "readv",
    "recvfrom",
    "sched_getaffinity",
    "sendto",
    "setenv",
    "setsockopt",
    "shutdown",
    "socket",
    "socketpair",
    "stat64",
    "statx",
    "write",
    "writev",
];
pub const LIBM_FNS: &[&str] = &[
    "hypotf",
    "hypot",
    "lgammaf_r",
    "lgamma_r",
    "log1p",
    "log1pf",
    "tgamma",
    "tgammaf",
];
/*
"pthread_atfork",
"pthread_attr_destroy",
"pthread_attr_init",
"pthread_attr_setstacksize",
"pthread_create",
"pthread_detach",
"pthread_join",
"pthread_self",
"pthread_setname_np",
*/
