verb /endpoint/ - *description*
    - body option - **default** - *description*

GET /item/ - *gets a list of matching items*
    - count? - **30** - *maximum number of matches returned*
    - tags? - **none** - *matching tags*
    - name? - **none** - *matching sub-name*
    - description? - **none** - *matching sub-description*

GET /item/info - *gets all information of a queried item*
    - item_id - *id of item queried*

POST /item/ - *add new item to database*
    - name - *item name*
    - description - *item description*
    - damages? - **"No damages recorded"** - *description of item damages*
    - tags? - **empty list** - *list of tag is*

GET /item/availability/ - *gets whether item is available on given date*
    - item_id - *id of item queried*
    - start_date? - **today** - *start of queried borrow period*
    - end_date? - **today** - *end of queried borrow period*

GET /item/availability/full - *gits list of current booked dates*
    - item_id - *id of item queried*

POST /image/ - *add new image to database**
    - item_id - *item id*
    - description - *description of image for screen readers*
    - image - *actual image somehow idk*

GET /image/ - *get images of item*
    - item_id - **empty image** - *item_id*
    - count? - **5** - *maximum number of matching images to return*

GET /tags/ - *gets list of tags*
    - with? - **none** - *tags which include substring*
    
POST /tags/ - *add a new tag to database*
    - name - *tag name*
    - general - *whether or not the tag is generic or a specific item model (e.g. a specific set of plates)*

GET /tags/list/ - *return a list of items tagged with all specified tags*
    - tag_ids - *ids of queried tags*
    - available - *whether or not the returned item ids must be available*


//ignoring for now
GET /tags/count/ - *return a count of items tagged with all specified tags*
    - tag_id - *ids of queried tags*






GET /auth/login/ - *return user_id if user exists*
    - username - *username*
    - password - *hashed password*

POST /auth/register/ - *attempt to register user*
    - username - *unique username*
    - password - *hashed password*

POST /borrow/borrow - *request to borrow specified item*
    - item_id - *id of requested item*
    - user_id - *id of user requesting to borrow item*
    - borrow_date_start? - **today** - *start of borrow period*
    - borrow_date_end? - **today** - *end of borrow periodday** - *end of borrow period*
    - reason? - **0** - *reason for borrowing (0 - borrowed, 1 - under maintenence, 2 - out of circulation)*

POST /borrow/return - *return borrowed item*
    - borrow_id - *id of borrow occurance*
    - damage? - **none** - *description of damages occured*
    - return_date? - **today** - *date item was returned*

