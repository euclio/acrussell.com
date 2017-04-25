---
title: Video Games and Programming
date: "6:08pm 03/15/14"
categories:
  - programming
tags: []

It's somewhat of a stereotype that programmers like to play video games. Of
course, not all programmers enjoy games, but it's undeniable that the
demographics often overlap. As a child, I remember being fascinated not only by
video games, but also by the idea of the video game console. I was amazed that
my inputs on a controller were translated into animations on the screen. This
fascination quickly evolved into a desire to make my own games. As a first
grader, I talked to some friends on the bus and learned that many games were
programmed in C++. I even managed to buy a "C++ for Dummies" book (In fact, it
predates the C++98 standard, making it interesting from a historical
standpoint). Unfortunately, I was way over my head. Programming is difficult,
and without any guidance I didn't have the attention or the understanding to
follow through. I did not attempt programming again until
[much later][Learning to Program].

I wasn't the only kid that wanted to learn how to make games. Many people manage
to scratch that itch by using tools like [GameMaker] or [RPGMaker]. I've used
both of these tools, and I think that they both serve a great purpose, but they
didn't satisfy me. I, along with many others, wanted to know what actually made
a game work. I wanted to build a game from the ground up.

Unfortunately, game programming is hard, especially for those attempting to
learn to code at the same time. But what makes game programming hard for
beginners?

* Difficult Languages

    Oft-recommended languages for game development (C++, C#, and JavaScript) can
serve as a high barrier to entry due to their complicated syntax.

* High Overhead

    To make a non-trivial game, a programmer must use a game development library
or engine. While these libraries will make programming easier in the long run,
it is the initial effort to learn the library and API can be overwhelming.

* Graphics

    Game programming requires knowledge of various domains. Working knowledge of
graphics, storage techniques, and gaming terminology (e.g., sprites, textures,
etc.) are a requirement.

And that's not all. There are a number of factors that make game programming
difficult for experienced programmers as well.

* Performance

    In most cases, games are performance-critical. For a school-taught
programmer, the emphasis is on correctness, rather than performance. Therefore,
it is difficult for a programmer to write games that sacrifice correctness for
performance.

* Unfamiliar design patterns

    Games often require programming that might seem "hacky" or inelegant. One
example that comes to mind is designing a weapon or enemy system. In this case,
it might be tempting to design a class hierarchy, but this adds complexity
without much benefit. There's a great blog [Game Programming Patterns] that
discusses [this][Prototype Pattern] and similar issues at length.

What is the solution to this? Unfortunately I don't have a satisfactory answer.
Beyond the programming aspects of games, it is very difficult to design good
looking art, sounds, and gameplay for even a 2D game. However, I think that
there are a number of steps to be taken that will ease the learning process on
the programming side. I think that pushing low-syntax languages like Python will
help beginners to clear the initial hurdles of programming. Though some might
balk at the decision to shy away from faster languages, I think that
overemphasizing performance at the initial stages does more harm than good.
Python also has a number of easy-to-learn game development libraries that are
quite easy to install. I would venture so far as to include a basic game
development library in the Python standard library to avoid having to install
anything for beginners.

It's a shame that many prospective programmers are discouraged by the difficulty
of game programming. Programming is an extremely useful skill, and programmers
should ease the learning process for beginners as much as possible. That starts
with encouraging the initial desire to learn and attempting to remove any
roadblocks that stand in the way.

[Learning to Program]: http://www.acrussell.com/blog/2013/12/11/in-defense-of-ti-basic
[GameMaker]: http://www.yoyogames.com/studio
[RPGMaker]: http://www.rpgmakerweb.com/
[Game Programming Patterns]: http://gameprogrammingpatterns.com/
[Prototype Pattern]: http://gameprogrammingpatterns.com/prototype.html
