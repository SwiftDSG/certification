# Library Management Application

This is an application to manage all the book loans and the availability of each books that's available to a library that uses this application

There are two seperate implementation in this application, the first one is the front-end (using website framework called _nuxt_) and the second one being a back-end server to handle all the requests happened on this application (using rust and actix combination for its server runtime and MongoDB for its database)

## Front-end (cert)
In this part of the application, there are some folders that are important to this project, those are:
- `/assets`: for containing all the fonts used in this application
- `/components`: for reducing writing the same code all over again by reusing all the components used in this application
- `/composables`: the one's responsible for communicating with the back-end and managing state of the application
- `/plugins`: hand-crafted plugins to make development easier
- `/public`: to serve static assets (e.g images, icons, favicons, etc)
- `/types`: to define all the types used on this application

## Back-end (cert-server)
In this part of the application, there are some folders that are important to this project, those are:
- `/keys`: for containing the public & private keys used for hashing tokens
- `/src`: the main folder of this project
- `/src/models`: used for containing all the models and its implementation for this project
- `/src/routes`: used for defining all the available routes for receiving request from the front-end
