IMAGE_NAME="$1"

echo -n 'Removing old image... '

OLD_IMAGE=`docker images -q "$IMAGE_NAME"`
if [ -n "$OLD_IMAGE" ]; then
    echo
    docker rmi "$OLD_IMAGE"
else
    echo 'nothing found.'
fi
