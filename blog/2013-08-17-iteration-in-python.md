title: Iteration in Python
date: 4:32pm 08/17/12
categories: programmming
tags: python

Iteration is one of Python's greatest strengths. Once I learned its power, I 
haven't used an 'i' or a 'j' in ages. Before this summer, I was used to the 
power of the `in` operator, but I think that the beauty of Python's iteration 
lies in list comprehensions and generator expressions.

Consider a simple list comprehension:

```python
post_authors = [post.author for post in blog_posts()]
```

The list comprehension allows users to map a function over each element in 
another list. The new list will contain all of the authors of the post objects 
returned by the `blog_posts()` function.

What if I wanted to get only authors of posts written less than three weeks 
ago?

```python
from datetime import date, timedelta
def is_recent_post(post):
    return post.date < date.today() - timedelta(weeks=4)

post_authors = [post.author for post in blog_posts if is_recent_post(post)]
```

List comprehensions are a very powerful way to create complex lists while 
keeping a low syntax overhead.

Now, it's important to think about why lists are created in the first place.  
Often, there really isn't a need for the overhead of a mutable list object with 
all of its elements in memory at once. In fact, much of the time a list is 
created only to be iterated over once. This is the perfect situation for an 
iterator.  They keep a low memory overhead while retaining a list-like 
interface for the `in` keyword. Often, generator expressions are the way to go 
when there is no need to keep the entire list in memory or creating the list 
involved expensive operations like file I/O.

```python
def blog_posts():
    return (read_post(file_name) for file_name in file_names)
```

List comprehensions and generator expressions are two of my favorite features 
in Python. They have clean syntax and allow for easy manipulation of data in a 
functional way.
