# Bayou
 
this project is a rewrite of my older project https://github.com/uberfig/activity_playground

this project has taken a turn from its original goals and is primarily implimenting the [versia protocol](https://versia.pub/) with basic [activitypub](https://www.w3.org/TR/activitypub/) support. 

The lack of documentation on the extensions to activitypub by mastodon and various projects makes it quite the pain to set up things like signing 

reasons implimenting activitypub sucks
 - we just have a little [blog post](https://blog.joinmastodon.org/2018/06/how-to-implement-a-basic-activitypub-server/) to go off for signing unless we want to read the source code for mastodon
 - the blog post is incorrect as mastodon requires the message digest be signed
 - no one mentons anywhere that the algorithm needs to be inferred from the silly @context instead of making the sane design decision to just list the algorithm used beside the pem
 - infinitely nested objects is valid within activitypub
 - when you unfollow someone your instance sends an undo. when you follow them again it undoes the undo, you can see how silly this is
 - follow requests need to have an id that can be accessed like comon this is silly

down the road there may be support for other databases, but for the time being postgres is the main focus. since this project is still very very under development there will be frequent changes the database without migrations until we have the first alpha release.

for setting up your environment to run, check [environment setup](environment_setup.md)

all contributions welcome :3
