verb /endpoint/ - *description*
    - body option - **default** - *description*

GET /item/ - *gets a list of items*
    - size? - **30** - *maximum number of matches returned*
    - tags? - **none** - *matching tags*

POST /item/ - *add new item to database*
    - name - *item name*
    - description - *item description*
    - damages? - **"No damages recorded"** - *description of item damages*

GET /tags/ - *gets list of tags*
    - size? - **10** - *maximum number of matches returned*
    - with? - **none** - *tags which include substring*
    
POST /tags/ - *add a new tag to database*
    - name - *tag name*


