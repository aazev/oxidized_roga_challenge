# Use the official MySQL image from Docker Hub
FROM mysql:8.0

# Copy the MySQL configuration file into the Docker image
COPY my.cnf /etc/mysql/conf.d/

# Environment variables for MySQL
ENV MYSQL_DATABASE=oxidized-roga-challenge
ENV MYSQL_USER=roga
ENV MYSQL_PASSWORD=rogalabs
ENV MYSQL_ROOT_PASSWORD=rogalabs

# Expose the MySQL port
EXPOSE 3306
