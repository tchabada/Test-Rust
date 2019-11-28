FROM postgres

# Create the table on start-up
ADD schemas/users.sql /docker-entrypoint-initdb.d/
