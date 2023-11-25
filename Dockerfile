FROM eclipse-temurin:21-jammy

WORKDIR /code

RUN wget https://github.com/flix/flix/releases/download/v0.41.0/flix.jar \
    && sh -c "$(curl --location https://taskfile.dev/install.sh)" -- -d -b /usr/local/bin

ENTRYPOINT ["task", "run"]

