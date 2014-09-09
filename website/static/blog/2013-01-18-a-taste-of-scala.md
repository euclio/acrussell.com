title: A Taste of Scala
date: 10:00pm 01/18/13
categories: news
tags: cs
      scala

This past semester I took a course entitled "Domain-Specific Languages." I had
no idea what to expect when I signed up for it, but it ended up being one of
the most interesting classes that I have taken in college. Most of the course
involved writing my own DSL in my language of choice. My language, ChipLang, is
already on my Github and will be uploaded to my site soon. Since there is so
much to talk about last semester's project, I will be splitting the discussion
of my DSL into multiple blog posts. This post will be about Scala.

My professor picked Scala as the language of the class because it has a number
of useful tools for creating DSLs in addition to it not being taught in other
courses at Harvey Mudd. Once the class had implemented a number of assignments,
we were allowed to pick our own language to implement our personal DSL in. I
picked Scala because I wanted to take advantage of its native parser-combinator
library, and because I simply enjoyed the language.

Why do I like Scala? First of all, it seems to do away with a lot of the tedium
of programming. This is a praise often sung about Python, but I think it rings
true about Scala as well. Scala's "Hello World" is a bit more verbose than
Python, but it is worlds away from Java. Here's how I would implement it:

```scala
object HelloWorld extends App {
    println("Hello World")
}
```

There's a bunch of cool things going on in this program. First, note the lack
of semicolons. They are inserted implicitly, similarly to JavaScript, which
makes for less noise when developing. Of course, the programmer could put them
in if he or she desires. Also, notice the object keyword. If you were writing
this program in Java, it would be an anonymous class that extends App. App
itself is pretty convenient too. It implicitly defines a main method and
supplies a variable args that holds the command line arguments. Not bad for
three lines of code, and a heck of a lot better than this:

```java
public class HelloWorld {
    public static void main (String [] args) {
        System.out.println("Hello world");
    }
}
```

Another nice feature is that `.scala` files can be named anything you like, and
can contain all the classes and objects you want, while `.java` files must be
named after the single public class they contain.

I don't want to turn this post into Java bashing; in fact, I love Java. Which
is great, because Scala can interface with Java out of the box. My DSL uses the
`javax.sound.midi` library with no additional overhead. And of course, Scala
compiles to Java bytecode and runs on the JVM so it's not necessary for users
to install a new runtime.

Another thing I love about Scala is its unique blend of object-oriented and
functional programming paradigms. Some readers might gasp at the thought of
mixing paradigms so liberally, but I enjoy it. Sometimes it's convenient to
think of my program as a collection of objects, and other times I want to
pattern match, filter and map. Scala allows me to do all of those things in the
same program. Its syntax for anonymous functions is amazing. There aren't many
languages out there where squaring

Lastly, Scala allows for some pretty helpful tools for DSLs. Internal DSLs are a
breeze to implement due to Scala's generous syntax. In most situations, the
method selector and the parenthesis of nullary functions are optional.
[ScalaTest][scalatest], a unit-testing library for Scala, is a DSL with such a
syntax. As for external DSLs, the Scala standard library comes with a number of
Parsers, included the combinator-parsers I mentioned before.

There's so much more I could talk about, but this post is already much longer
than intended. I might write another post down the line about more cool things
in Scala, like for expressions. Scala is one of my new favorite languages to
develop in. Though it has its share of problems (syntactic ambiguities, easily
broken backwards-compatibility, and type quirks), to me its advantages far
outweigh its flaws.

[scalatest]: http://www.scalatest.org/
