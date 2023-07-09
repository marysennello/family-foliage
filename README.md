API:

GET /api/tree/:id
	// Gets this person and their parents and children
	{ "full_name": "{full_name}", "id": "{id}", "children": [{"full_name": "{full_name}", "id": "{id}"}, ...], "parents": [{"full_name": "{full_name}", "id": "{id}"}, ...] }

GET /api/tree/
	// Gets the entire tree, same format as above, but with everyone. I'd imagine that this would be used for an initial load of the page? But perhaps this needs reconsideration

GET /api/bio/:id
	{ "full_name": "{full_name}", "id": "{id}", "bio": "{bio_blob}" }

POST

Data (DynamoDB):

 people table:
	{ "id": "{id}", "full_name": "{full_name}", "bio": "{bio_s3_link}", "children": [id, ...], "parents": [id, ...], "spouses": [id, ...] }


The backend (written in Rust) will be responsible for translating the data into the API. If the frontend requests GET /tree, the backend will have to iterate over the id, children IDs, and parent IDs to get all the people's information. The frontend will present that info nicely with all the necessary link to the bio.

If the front end requests /bio/:id, the bio information can have identifiers to other people (as tags). How should we format this? Hyperlinks would make it easiest, but have limited portability... if the API changes, we'd have to update people's bios. Perhaps just a string like <person-ref>{id}</person-ref> which is pretty tough to accidentally type as part of their real bio. This means you'd have to translate all instances of <person-ref> to an actual hyperlink dynamically.

Haven't talked about POSTs to handle 1) updating the bio, and 2) the more difficult task of updating the tree. For #1, it's just a POST to /api/bio/:id. For #2, a few POSTs are needed...

To add a new person:
POST /api/tree/
	{ "full_name": "{full_name}" }

To link people:
/api/tree/:id/children/:child_id

This would have to automagically populate both the parent of :child_id and the child of :id

The same thing would be true for

/api/tree/:id/parents/:parent_id

And also

/api/tree/:id/spouses/:spouse_id

And then all of these would have to be DELETEs, too, so /api/tree/:id would also need a DELETE.

