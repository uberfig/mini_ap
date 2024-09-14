# mini_ap
 
this project is a rewrite of my older project https://github.com/uberfig/activity_playground

this project has taken a turn from its original goals and is primarily implimenting the [versia protocol](https://versia.pub/) with basic [activitypub](https://www.w3.org/TR/activitypub/) support. 

The lack of documentation on the extensions to activitypub by mastodon and various projects makes it quite the pain to set up things like signing (for example we just have a little [blog post](https://blog.joinmastodon.org/2018/06/how-to-implement-a-basic-activitypub-server/) to go off for signing unless we want to read the source code for mastodon. to make it worse, the blog post is incorrect as mastodon requires the message digest be signed as well)

down the road there may be support for other databases, but for the time being postgres is the main focus. since this project is still very very under development there will be frequent changes the database without migrations until we have the first alpha release.

for setting up your environment to run, check [environment setup](environment_setup.md)

all contributions welcome :3
