#!/bin/bash
mongosh  -u $MONGO_INITDB_ROOT_USERNAME -p $MONGO_INITDB_ROOT_PASSWORD --eval "\
    db = db.getSiblingDB('admin'); 
    db.createUser({ user: '$MONGO_INITDB_USERNAME', pwd: '$MONGO_INITDB_PASSWORD', 
        roles: [
            { 
                role: 'readWrite', 
                db: '$MONGO_INITDB_DATABASE' 
            }
        ] 
    });
    "