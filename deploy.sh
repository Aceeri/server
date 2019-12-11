#/bin/sh
set -e

rev="$(git log -1 --date=format:%F-%H%M --pretty=format:'%cd-%h')"
image=gcr.io/aceeri/webserver
full_image=$image:$rev

echo "Building new image $full_image";
docker build -t $full_image .
docker save $full_image | bzip2 | pv | \
    ssh $1 'bunzip2 | sudo docker load'
