verb /endpoint/ - *description*
    - body option - **default** - *description*

GET /item/ - *gets a list of items*
    - count? - **30** - *maximum number of matches returned*
    - tags? - **none** - *matching tags*
    - name? - **none** - *matching sub-name*
    - description? - **none** - *matching sub-description*

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
    - count? - **10** - *maximum number of matches returned*
    - with? - **none** - *tags which include substring*
    
POST /tags/ - *add a new tag to database*
    - name - *tag name*
    - general - *whether or not the tag is generic or a specific item model (e.g. a specific set of plates)*

GET /tags/list - *return a list of items tagged with all specified tags*
    - tag_ids - **ids of queried tags**
    - available? - **false** - *whether or not the returned item ids must be available*

GET /tags/count - *return a count of items tagged with all specified tags*
    - tag_id - **ids of queried tags**


