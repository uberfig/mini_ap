# mini_ap
 
this project is a rewrite of my older project https://github.com/uberfig/activity_playground

this project intends to provide a simple and robust set of utilities for the [activitypub protocol](https://www.w3.org/TR/activitypub/) including endpoints, rust structs for serializing and deserializing activitystream objects, a basic database setup, signing and verifying posts, and sending out posts. This project will also have a minimal example server to test the implimentation. Down the road I plan to fork this project for a more usable/production setting and I also plan to fork it for a federated blog hoster 

this project intends to be more stable with unit testing for activitystream types, tokio-postgres instead of sqlx, and refinery for built in migrations. This project may also include the option to use sqlite instead in the more distant future

for setting up your environment to run, check [environment setup](environment_setup.md)
