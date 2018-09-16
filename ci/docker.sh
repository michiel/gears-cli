set -ex

main() {
    echo $DOCKER_PASS | docker login -u $DOCKER_USER --password-stdin
    # - docker login -e $DOCKER_EMAIL -u $DOCKER_USER -p $DOCKER_PASS
    export REPO=gearsproject/gears
    export TAG=$TRAVIS_BRANCH
    echo $REPO
    echo $TAG
    echo $TRAVIS_COMMIT
    # - docker build -f Dockerfile -t $REPO:$COMMIT .
    rm -rf ~/.cargo/registry
    rm -rf ~/.cargo/git
    docker build . -f Dockerfile -t $REPO:$TRAVIS_COMMIT
    docker tag $REPO:$TRAVIS_COMMIT $REPO:$TAG
    docker tag $REPO:$TRAVIS_COMMIT $REPO:travis-$TRAVIS_BUILD_NUMBER
    docker push $REPO
}

echo "Checking for TARGET - $TARGET"
# if [ "$TARGET" == "x86_64-unknown-linux-musl" ] && [ -z $TRAVIS_TAG ]; then
# if [ "$TARGET" = "x86_64-unknown-linux-musl" ]; then
if [ "$TARGET" == "x86_64-unknown-linux-musl" ] && [ "$TRAVIS_PULL_REQUEST" == "true"]; then
    echo "TARGET matched for docker build"
    main
else
  echo "TARGET not matched for docker build"
fi
