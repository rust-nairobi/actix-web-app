# Cloud Environment Manager

This is an application written in Rust based on actix-web framework. We are using postgres as the database and diesel as the ORM

# About this projects
It meant to manager how devs are using testing and staging environments, showing who owns what environment


# Preliquisite
- Diesel provides a separate CLI tool to help manage your project. Ensure the diesel cli is installed using `cargo install diesel_cli`
- Create a .env directory if it does not exist and set database URL
`echo DATABASE_URL=postgres://username:password@localhost/your_database_here > .env`
- Now Diesel CLI can set everything up for us. `diesel setup`

# How to run
- setup postgres database and ensure the same name is set on the .env file `DATABASE_URL=postgres://username:password@localhost/your_database_here`
- Run `diesel setup` to run migrations
- open terminal and run `cargo run`
The application may take small time to first build before running your web server
- Using the browser `http://localhost:8083/` you should see a static page.
- Use postman or any other Rest client to test your server api endpoint as below
endpoint 1: `http://127.0.0.1:8083/api/usersenv_list`
method: `POST`

expected output:
`{
    "status": 200,
    "message": "usersenv_list result.",
    "usersenv_list": [
        {
            "id": 1,
            "user_id": 2,
            "env_id": 2,
            "po_id": 3,
            "start_date": "2018-07-21T23:41:45.672806",
            "max_duration": "12hrs",
            "lease_period": "from 24th Aug to 25th Aug 2018",
            "status": 1,
            "env_type": "EB",
            "created_at": "2018-07-21T23:41:45.672806"
        }
    ]
}`


try other endpoints:
- `http://127.0.0.1:8083/api/env_list`
- `http://127.0.0.1:8083/api/po_list`
- `http://127.0.0.1:8083/api/product_list`

# TODO
- Authenticate endpoints
- separate routers from the server
- modify the error logs to be more descriptive
- Complete the Docker setup to run the server well

