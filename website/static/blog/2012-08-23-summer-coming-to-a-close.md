title: Summer Coming to a Close
date: 10:28pm 08/23/12
categories: news

As I get ready to start my sophomore year of college I thought that I should
write a bit about my job at Washington University. I worked for the computer
science department for eleven weeks, comprising most of the summer. I made some
good friends at my job and I hope to keep in contact with them. Overall the
work was a very positive experience. I worked under a professor and one of his
graduate students on a research project. I mostly wrote scripts to parse
biological data and run analysis on results. My adviser is currently drafting a
paper that details our findings.

While I can't say exactly what the project involved because the research is
unpublished, I can confidently say that I learned a lot of things about both
biology and computer science. A quick list of the skills that I acquired that
are relevant to this blog:

* Feeling at home in vim
* Python experience
* bash scripting

I'll start with detailing vim  my favorite tool to program with. I started
using vim first semester and have since fallen in love with it. I had very
little experience with the command line before college and my classes
tentatively introduced me to commonly used tools. At first I found it difficult
and a little bit scary to use the command line instead of a GUI. I certainly
never expected myself to embrace a command line text editor! Despite my initial
hesitance, it never ceases to amaze me how powerful a tool vim truly is. Since
many of my scripts needed to be run on a server that had no graphical
interface, I decided that this was the prime opportunity to improve my vim
skills. There are three things I did that vastly improved my workflow. First of
all, I disabled the arrow keys completely in my `.vimrc` using

```vim
noremap <up> <nop>
noremap <down> <nop>
noremap <left> <nop>
noremap <right> <nop>
inoremap <up> <nop>
inoremap <down> <nop>
inoremap <left> <nop>
inoremap <right> <nop>
```

At first this addition made me a lot slower, because I would press the arrow
keys every once in a while out of habit. However, eventually the tweak forced
me to become comfortable in normal mode because I had no "easy" way to move
around (I put "easy" in quotation marks because the arrow keys are woefully
inefficient when it comes to vim's movement commands).

My reluctance to enter normal mode came partly from the inconvenience of
hitting the Esc key. On the terminal that vi (vim's predecessor) was programmed
for, the Esc key resided where the Tab key is located today. Therefore, it was
a logical key to press when one wants to stop typing and enter commands because
it was close to the home row. Today, the Esc key is very far away from the home
row and requires a uncomfortable stretch of the left hand that is prone to
missing when touch typing. An alternative to hitting Esc is emitting the ASCII
control character `^[`, that is, typing Ctrl+[. Unfortunately, the control key
has also moved; it used to be where Caps Lock is today! Since this technique
still involves pressing another key from the far reaches of the keyboard, I was
not satisfied. But there is a solution! In fact, it brings vim a little closer
to its roots. The solution is a simple OS change to make a press of Caps Lock
register as a Ctrl. This change allowed me to press Caps Lock+[ instead of Esc.
Both keys are very close to the home row which allows me to enter normal mode
quickly. I began to use hjkl instead of the arrow keys. I've become much faster
than I ever was before. I now had opportunity to use more opportunity to use
movement commands like w, o, and s that are much faster than navigating the
document with arrow keys.

As much as I love my text editor, the meat of my work was written in Python.
Though I had only dabbled in Python before this project, I had the opportunity
to learn many things about the language. I now consider myself proficient in
Python and quit familiar with coding large scripts with it. But why Python?

I picked Python for a variety of reasons. First of all, it's interpreted, not
compiled. This allowed me to run my scripts right after changing a single line,
with no wait. It also allowed me to debug more easily because I could figure
out why certain lines were giving me trouble by pasting them into the Python
shell. Secondly, the language is very high level. It makes resource management
a breeze. For example, this Java code to read a file and print out the each
line and its number:

```java
Scanner reader = null;
try {
    reader = new Scanner(new BufferedReader(new FileReader("data.txt")));
    int count = 1;
    while (reader.hasNextLine()) {
        System.out.println(count + ". " + reader.nextLine());
        count++;
    }
} finally {
    if (reader != null) { 
        reader.close();
    }
}
```

Java 7 has made this kind of set up and tear down of resources a little bit
less verbose, but the platform has yet to be widely adopted as of now.
Regardless, the Python version of this program is far more elegant:

```python
with open('data.txt', 'r') as f:
    for count, line in enumerate(f):
        print('{0}. {1}'.format(count, line))
```

High level features like complex string formatting and context managers drew me
especially to Python for this project. I also used list comprehensions to
traverse and modify my data in one step. Working with the language was a breeze
and allowed me to focus my thinking on the best way to implement each step
instead of worrying about the quirks of my chosen language.

While vim and Python weren't completely new to me, bash scripting was a
completely new endeavor. I wrote most of my scripts in Python, I tied them all
together in a bash script to submit to a cloud server. Bash is a fun little
language, and I enjoyed writing it. It allowed me to get familiar with a lot of
UNIX programs. I used awk to do pre-processing on the data to simplify the
parsing that my Python scripts had to do. Since many of the biological programs
I used required long lists of parameters, I used the optargs utility to give my
script parameters of its own: I could choose which parts of the script to run
each time. The script allowed me to run the different programs with wildly
different parameter lists by adding a single letter to the script's own
arguments. My favorite part about bash? 'if' statements must be terminated with
'fi'; 'case' statements an 'esac' :).

Overall the summer was filled with a lot of learning about programming and the
tools that I have available to me. I've been inspired to work on a new program
in Python, and can finally call myself truly at home in UNIX. I can't wait to
see what this next semester has in store for me.
