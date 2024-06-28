# RusticTime
RusticTime is a calendar application that is written in rust. 

Todo:
* Write back end
  - [ ] Implement sqlite database
      - [X] Create
          - [X] add dates to struct
      - [X] Read / Fetch
      - [ ] Update
          - [ ] add option enum to all task feilds
          - [ ] update selected task with fields not none in given task
      - [ ] Delete
  - [ ] Implement Event sorting / prioritizing algorithm
      - [X] Sort by priority (done through DB)
      - [ ] Sort by due date
      - [ ] Order tasks in a list
* Create front end
* Implement CalDav
