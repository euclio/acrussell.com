title: In Defense of TI-Basic
date: 3:21pm 01/14/2014
categories: programming

[Edsger Dijkstra], famed Computer Scientist, once said "It is practically
impossible to teach good programming to students that have had a prior exposure
to BASIC: as potential programmers they are mentally mutilated beyond hope of
regeneration." Of course, Dijkstra didn't mean that all BASIC programmers are
essentially bad, but that BASIC introduced bad practices into their style that
were difficult to rectify later. Dijkstra also wrote the famous essay [Go To
Statement Considered Harmful], a criticism of a statement held dear to many
BASIC programmers.

However, this post isn't meant to be about the merits or demerits of BASIC. I
want to talk about a similar language: TI-BASIC, the programming language
included on certain classes of TI graphing calculators. The language is very
likely familiar to students who used a TI-83 or TI-84.

I must admit that this language is very special to me: it's essentially my first
experience with real programming. I did have some experience with LOGO and Lego
Robotics when I was a kid, but I wouldn't necessarily consider those
"programming" in the sense of the word that most people are accustomed to. I
even attempted to purchase "C++ For Dummies" in the first grade (of course, that
was way over my head!). I really had my first taste of programming with
TI-BASIC. I remember sitting in Algebra class in 8th grade (where I first was
able to use a graphing calculator) and seeing the mysterious `PRGM` key. I was
fascinated by the concept of writing programs that could do things like solve
quadratic equations.

Programming on a TI-83 was an experience, to put it in nice words. The device
has a small screen, user-unfriendly keyboard, and the TI-BASIC interpreter is
slow. When I started using the language, I used `GOTO` statements almost
exclusively. Dijkstra's belief must have seemed to be true.

Although, one day, I reached a strange bug when I used a `GOTO` to exit a
for-loop. It seemed like the loop counter variable would retain its previous
value the next time that the loop was entered, which is the opposite of what I
expected. I searched the problem online, and I discovered a forum post where
another programmer was having the exact same issue that I was having. There was
a reply with a stern warning to never use `GOTO`, and instead use more
structured programming techniques.

Another problem that I often ran into was the issue of creating a "friendly
window", or a window where the graph screen was equal to pixels. This was
necessary because the functions used to draw to the screen were split into
graph-based and pixel-based. By using a friendly window, a programmer can pass
the same arguments to both functions. Since I often found myself writing the
same code over and over, I looked for a way to save myself the typing. There are
no user-defined functions in TI-BASIC, but a programmer can call other programs
from within the same program. Thus, I made a program that set a friendly
graphing window and then called that from within whatever program I wanted to.

There were more problems with TI-BASIC from a programmer's perspective. It only
had 26 variables (A-Z, though it is possible to get more variables by using
lists or other creative techniques the alphabetic variables were easiest to
use). Also, it was not possible to indent code, and lines often wrapped around
the screen due to the short screen width. Therefore, it became rather obvious
early on that I had to document my program clearly and concisely. TI-BASIC
itself has no comments, but one may use a string on a line by itself as a sort
of pseudo-comment.

Though TI-BASIC is clearly a language with its flaws, I think that these flaws
in turn taught me good programming design principles by realizing why such
things are necessary. When I decided to move onto a more user-friendly language
like Java, I knew the importance of structured programming, abstraction, and
documentation.

I still have a few of my old TI-BASIC games lying around somewhere. Maybe
someday I'll put a link to them somewhere. I look back fondly on my time spent
on the TI-83, and I hope that my code might serve as a resource for another
budding programmer.

[Edsger Dijkstra]: http://en.wikipedia.org/wiki/Edsger_Wybe_Dijkstra
[Go To Statement Considered Harmful]: http://www.u.arizona.edu/~rubinson/copyright_violations/Go_To_Considered_Harmful.html
