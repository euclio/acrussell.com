title: Massive Updates!
date: 10:41am 05/25/12
categories: news
            projects
            website
tags: css
      doodler
      html
      java

With the summer here, I've had a lot more time to give to my personal projects.
I've made two big updates to the site and to Doodler. I've updated a lot, so
brace yourself for my longest blog post yet! First of all, I made a few
improvements to acrussell.com. I've been reading about user accessibility and
how my web design can better accommodate a wide range of users. I took my
navigation bar and made it easier for a user of my site to tell which page they
are on at a glance. Interestingly, I was researching what the best method to do
this was, and I found that it was possible to accomplish in pure CSS. This
means that the coloring is easily changed in the CSS stylesheet and I don't
have to worry about any server-side "magic" going on behind the scenes. There's
still more I want to improve, especially making semantic HTML a priority. This
will include reformatting my navigation and social links as lists instead of
paragraphs.

I also made two visual improvements to pages. I updated the 404 (Not Found)
page with a new picture, and made the design mesh a little better with the rest
of the site. I overhauled the look of the projects page, and hopefully made it
look a little less temporary. The CSS of the site still leaves a little to be
desired. I unfortunately think it feels a little "hacky." I also want to get
rid of my remaining inline styles and mold them into my main stylesheet. I'll
be ironing these issues in the weeks to come.

Now for the big one: I am very excited to announce an entirely new feature for
the Doodle program! There is now a "Tools" pane at the top of the program, and
it holds two buttons: the Pen and Stamp. The Stamp tool functions just like the
old program did: it draws a series of shapes onto the canvas. The Pen, however,
functions more closely to what is expected of a drawing program. No matter how
fast you draw, a smooth line will always follow where the mouse goes. When I
first implemented these features, I stumbled upon an interesting bug. To
explain the bug, I will have to explain a little about how the rendering of the
canvas is implemented. Bear with me!

Right now, Doodler depends on two threads: the event-dispatch thread, and a
"drawing" thread. The event-dispatch thread's job is to draw the user interface
and fire mouse events like clicking, dragging, and pressing. When these events
are fired, they send the point where they were fired to the drawing thread,
where the point is pushed onto a blocking queue. The drawing thread's job is to
wait until the mouse is pressed, then constantly pop points off of the queue
and "render" the currently selected tool. In the case of the Pen, the drawing
thread uses the last point and the current point and connects them with a
stroke defined by the size slider. In the case of the stamp, it simply draws a
shape of the given size at each point. Blocking queues are perfectly situated
for this kind of problem; they have a method that allows the drawing thread to
stay put until the next point has been pushed onto the queue.

Now here's where the bug comes in: I didn't make the drawing thread wait. What
happened was that when a user switched their tool, then the drawing thread was
stuck waiting for another point to come into the blocking queue, while also in
the rendering code for the wrong tool.  This led to a strange bug where the Pen
lines would start with a Stamp. Finding the source of the bug wasn't easy, and
it sure didn't help that multi-threaded code is nearly impossible to debug:
debugging tools make the threads run at different speeds than in normal
execution. Thankfully fixing this bug also allowed me to fix a race condition
in the drawing thread initialization, which a much more serious bug.

Overall, I am very satisfied with the progress that I have made. I am really
proud of how my website looks, and Doodler is starting to feel more and more
like a fully featured program. This summer should see many more improvements
and hopefully a new project soon. Thanks for reading!
