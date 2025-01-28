FROM gcc:14.2.0

WORKDIR /var/www/cpp

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install sudo nano ccache clang -y
RUN apt-get install libjsoncpp-dev -y

COPY ./.gitignore ./.gitignore
COPY ./LICENSE ./LICENSE
COPY ./Makefile ./Makefile
COPY ./README.md ./README.md

EXPOSE 3001