title: Patching Vim
date: 8:39pm 05/09/14
categories: programming
            open source

As those familiar with me probably know, I'm a huge fan of [Vim], an open-source
text editor that has been around since 1991. I use the editor almost every day,
and though I've become very familiar with the program, I still come across new
features and functions almost weekly. Vim is an extremely important part of my
workflow, and I had been looking for a way to contribute to the project. I'm
happy to announce that earlier this week I wrote a patch which was officially
accepted into Vim upstream! This post explains the process that I took to submit
my patch, and hopefully it shows how easy it is to contribute to open source
software.

Specifically, my patch adds support for [fish], the __f__riendly __i__nteractive
__sh__ell to Vim. I use fish as my default shell, so I use it just as much if
not more than Vim. Fish is a fully featured shell that aims to be as
user-friendly as possible. That goal is accomplished in part by avoiding
extraneous syntax that can be difficult-to-remember ([I'm looking at you,
bash][bash pitfalls]). This means that fish makes a number of
backwards-incompatible changes, including introducing a completely new scripting
language, and is unable to run bash scripts. This might seem like a deficiency,
but I have found that the benefits of fish outweigh the problems that I've run
into.

One of the problems that I encountered while using fish is that a lot of my Vim
plugins refuse to work with the shell. The plugins would either crash or print
unhelpful error messages (if any error message at all). Most of the plugin
authors would recommend putting `set shell=/bin/bash` in the `.vimrc` as a
workaround. I wasn't happy with this solution because it was a cop-out. Vim
strives to work for almost all configurations, and it seems like a major
oversight that it didn't support a major shell.

One of my plugins returned a message that my "shell doesn't support UNIX-style
redirects." So, I thought that the issue stemmed from the fact that fish has
different syntax for redirects than bash. Where a bash scripter might write

```sh
# Redirect both stdout and stderr to a file
echo "hello" 2>&1 hello.txt
```

A fish scripter might write

```fish
# Redirect both stdout and stderr to a file
echo "hello" > hello.txt ^ &1
```


I knew that Vim automatically detects which shell as user is using and sets
variables such as `shellredir` and `shellpipe` to appropriate values for the
shell, which are then used to construct commands. I assumed that fish simply
wasn't included in this auto-detection and therefore the variables were being
set incorrectly. I was right that Vim doesn't automatically detect fish, and
instead assumes a bash-style shell. However, fish *does* accept bash-style
redirects, so this wasn't the issue. I decided to examine into the plugins' code
and see where exactly Vim was failing.

With verbose output on, it turns out that the plugin was failing when it called
the `system()` builtin. This function spawns a shell to run external commands,
pipes the result into a temporary file, and then displays the contents of that
file. The function should have worked, but it instead was returning E484:
"cannot read temporary file". Clearly something was wrong with this function
internally.

Now here's where I dove into Vim's source. This was a rather intimidating
process: Vim's code base is huge and peppered with `#ifdef` and `#define`s to
support all of the platforms that Vim runs on. I looked for the place where the
`system()` builtin was parsed, and found that it called another function
internally to construct the command. And here's where the problem lied! Vim
allocated a buffer large enough to store the command inside a subshell. For
instance, if a user executed `:call system("echo hello")` from Vim, the program
would allocate a buffer with the contents `(echo hello)`. While almost all
shells support this syntax, *fish doesn't*! The parentheses cause a syntax
error, so the command fails every time. The fix for this was rather simple: I
automatically detect the name of the shell, if it's fish, then we use the fish
syntax `begin; echo hello; end` instead of the parenthesis syntax. This allows
the shell to pipe the output of a series of commands into the temporary file.
The detection code already existed in another part of the codebase, so I just
moved that code into its own function to avoid code duplication.

I submitted the patch eager for review. However, within a week it was accepted!
I was ecstatic. Not only had I contributed to a major codebase, but I added a
new feature that hopefully will save users headaches in the future. Though the
codebase was initially rather daunting, I found that it was only necessary to
understand the part of the project that I was attempting to contribute to. The
actual patch submission was very quick and painless, and the users on the
mailing list were very helpful.

My advice to people wanting to contribute to open source is simple: do it! Find
a bug that really annoys you or a feature that you'd really like to have, clone
the repository, and make your patch. The worst that can happen is that you get
your patch rejected. Most people will be very helpful in review, and if not, you
can always maintain the patch for yourself. The hardest part about contributing
is working up the courage to dive into the codebase and submit the patch.

If you're curious, the patch that I submitted may be found [here][patch].

[Vim]: http://vim.org
[fish]: http://fishshell.com
[bash pitfalls]: http://mywiki.wooledge.org/BashPitfalls
[patch]: https://groups.google.com/forum/#!topic/vim_dev/bNfWJaM6DuY
