const Api_V1 = {
    docker:{
        image:{
            pull: "/api/v1/docker/image/pull",
            remove: "/api/v1/docker/image/pull",
            list: "/api/v1/docker/image/pull",
        },
        volumes:{
            list: "/api/v1/docker/volumes/list",
            delete: "/api/v1/docker/volumes/delete",
            create: "/api/v1/docker/volumes/create",
        }
    }
}