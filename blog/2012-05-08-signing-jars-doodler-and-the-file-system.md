---
title: "Signing JARs: Doodler and the File System"
date: "11:37pm 05/08/12"
categories:
  - news
  - projects
tags:
  - doodler
  - java

I'm happy to announce that Doodler is now able to save and load images from a
user's computer. This was accomplished by "signing" the JAR file. This is
required in order for the program to request the needed I/O permissions.
However, since I signed the file myself, the Java Virtual Machine complains
that there is no guarantee the file is actually mine. But you trust me, right?
In order to have a fully signed JAR I would need to have an independent firm
like VeriSign look at the file, but this is unnecessary for my humble projects.
Thus, you might see a dialog warning you about the file. I consider this a
small price to pay for Doodler to have its full functionality.
