# Booky 

An app to create several bookmarks to websites

- User can search in the address bar using a command, if that command exists, 
user gets redirected to that smart bookmark, else, redirect to a normal google search

- For now I'll create a Github and Twitch bookmark, and you use them as so:
you type "gh Gabriel2233/hai". Under the hood, we will be calling a local webserver that listens to 
this requests in a search route. If the first half of the command is known (Github, gh) for example, 
I'll redirect to that page. If we have a search that is "how to rust?", that's not a known command, so we redirect 
to a google search

- Dev process

I'll have two routes set up, a index route, that's just going to serve the home path so we don't get a "cannot 
GET /", and a "/search" route, where we have a query param named cmd ("/search?cmd=gh some/repo")

I cna then parse this request, match against known values and redirect to the correct route
